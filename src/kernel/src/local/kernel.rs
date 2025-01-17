// Copyright 2021 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use futures::TryStreamExt;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    async_trait, Bucket, Error, KernelUpdate, Result, ResultStream, Sequence, Stream, Version,
    VersionUpdate,
};

#[derive(Clone)]
pub struct Kernel<S: Stream, B: Bucket> {
    inner: Arc<Mutex<Inner>>,
    stream: S,
    bucket: B,
}

struct Inner {
    current: Arc<Version>,
    updates: broadcast::Sender<Arc<VersionUpdate>>,
    last_sequence: Sequence,
}

impl<S: Stream, B: Bucket> Kernel<S, B> {
    pub fn new(stream: S, bucket: B) -> Self {
        let (tx, _) = broadcast::channel(1024);
        let inner = Inner {
            current: Arc::new(Version::default()),
            updates: tx,
            last_sequence: 0,
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
            stream,
            bucket,
        }
    }
}

#[async_trait]
impl<S: Stream, B: Bucket> crate::Kernel for Kernel<S, B> {
    type Bucket = B;
    type Stream = S;

    async fn stream(&self) -> Result<Self::Stream> {
        Ok(self.stream.clone())
    }

    async fn bucket(&self) -> Result<Self::Bucket> {
        Ok(self.bucket.clone())
    }

    async fn apply_update(&self, update: KernelUpdate) -> Result<()> {
        let mut inner = self.inner.lock().await;
        inner.last_sequence += 1;
        let mut version_update = update.update;
        version_update.sequence = inner.last_sequence;
        // TODO: update current version
        inner
            .updates
            .send(Arc::new(version_update))
            .map_err(Error::unknown)?;
        Ok(())
    }

    async fn current_version(&self) -> Result<Arc<Version>> {
        let inner = self.inner.lock().await;
        Ok(inner.current.clone())
    }

    async fn version_updates(&self, _: Sequence) -> ResultStream<Arc<VersionUpdate>> {
        // TODO: handle sequence
        let inner = self.inner.lock().await;
        let stream = BroadcastStream::new(inner.updates.subscribe());
        Box::new(stream.map_err(Error::unknown))
    }
}
