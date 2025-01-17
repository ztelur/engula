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

use std::collections::HashMap;

/// An increasing number to order versions.
pub type Sequence = u64;

/// The state of a kernel at a specific time.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Version {
    pub sequence: Sequence,
    pub meta: HashMap<String, Vec<u8>>,
    pub objects: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VersionUpdate {
    pub sequence: Sequence,
    pub set_meta: HashMap<String, Vec<u8>>,
    pub delete_meta: Vec<String>,
    pub add_objects: Vec<String>,
    pub delete_objects: Vec<String>,
}
