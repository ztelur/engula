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

use engula::engine::hash::{Engine, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let engine = Engine::new();
    let key = vec![1];
    let value = vec![2];
    engine.set(key.clone(), value.clone()).await?;
    let got = engine.get(&key).await?;
    assert_eq!(got, Some(value));
    Ok(())
}
