// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::slice;

#[cfg(target_os = "windows")]
include!("pkcs11_windows.rs");

#[cfg(target_os = "windows")]
pub const CK_UNAVAILABLE_INFORMATION: u32 = std::u32::MAX;

#[cfg(not(target_os = "windows"))]
include!("pkcs11_unix.rs");

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CK_ATTRIBUTE {
    pub type_: CK_ATTRIBUTE_TYPE,
    pValue: CK_VOID_PTR,
    pub ulValueLen: CK_ULONG,
}

impl CK_ATTRIBUTE {
    pub fn value(&self) -> &[u8] {
        if self.pValue.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.pValue as *const u8, self.ulValueLen as usize) }
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        if self.pValue.is_null() {
            self.ulValueLen = value.len() as CK_ULONG;
            return;
        }
        if (self.ulValueLen as usize) < value.len() {
            self.ulValueLen = CK_UNAVAILABLE_INFORMATION;
            return;
        }
        unsafe { slice::from_raw_parts_mut(self.pValue as *mut u8, value.len()) }
            .copy_from_slice(&value);
        self.ulValueLen = value.len() as CK_ULONG;
    }
}
