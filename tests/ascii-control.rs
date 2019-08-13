// include-flate
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(proc_macro_hygiene)]

include!("../test_util.rs");

use include_flate::flate;

flate!(pub static DATA: str from "assets/ascii-printable.txt");

#[test]
fn test() {
    verify_str("ascii-printable.txt", &DATA);
}
