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

//! A [`Kernel`] implementation that stores everything in memory.
//!
//! [`Kernel`]: crate::Kernel

mod kernel;

pub mod mem;

pub use self::kernel::Kernel;

#[cfg(test)]
mod tests {
    use futures::TryStreamExt;

    use super::mem;
    use crate::*;

    #[tokio::test]
    async fn update() -> Result<()> {
        let kernel = mem::Kernel::default();

        let handle = {
            let mut expect = VersionUpdate {
                sequence: 1,
                ..Default::default()
            };
            expect.set_meta.insert("a".to_owned(), b"b".to_vec());
            expect.delete_meta.push("b".to_owned());
            expect.add_objects.push("a".to_owned());
            expect.delete_objects.push("b".to_owned());
            let mut version_updates = kernel.version_updates(0).await;
            tokio::spawn(async move {
                let update = version_updates.try_next().await.unwrap().unwrap();
                assert_eq!(*update, expect);
            })
        };

        let mut update = KernelUpdate::default();
        update.set_meta("a", "b");
        update.delete_meta("b");
        update.add_object("a");
        update.delete_object("b");
        kernel.apply_update(update).await?;

        handle.await.unwrap();
        Ok(())
    }
}
