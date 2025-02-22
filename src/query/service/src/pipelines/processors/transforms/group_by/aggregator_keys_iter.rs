// Copyright 2022 Datafuse Labs.
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

use std::slice::Iter;

use common_arrow::arrow::buffer::Buffer;
use common_exception::Result;
use common_expression::types::number::Number;
use common_expression::types::string::StringColumn;
use common_expression::types::string::StringIterator;

use super::large_number::LargeNumber;

pub trait KeysColumnIter<T: ?Sized> {
    type Iterator<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;

    fn iter(&self) -> Self::Iterator<'_>;
}

pub struct FixedKeysColumnIter<T: Number> {
    column: Buffer<T>,
}

impl<T: Number> FixedKeysColumnIter<T> {
    pub fn create(column: &Buffer<T>) -> Result<Self> {
        Ok(Self {
            column: column.clone(),
        })
    }
}

impl<T: Number> KeysColumnIter<T> for FixedKeysColumnIter<T> {
    type Iterator<'a> = Iter<'a, T> where Self: 'a, T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.column.iter()
    }
}

pub struct LargeFixedKeysColumnIter<T: LargeNumber> {
    holder: Buffer<T>,
}

impl<T: LargeNumber> LargeFixedKeysColumnIter<T> {
    pub fn create(holder: Buffer<T>) -> Result<Self> {
        Ok(Self { holder })
    }
}

impl<T: LargeNumber> KeysColumnIter<T> for LargeFixedKeysColumnIter<T> {
    type Iterator<'a> = Iter<'a, T> where Self: 'a, T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.holder.iter()
    }
}

pub struct SerializedKeysColumnIter {
    column: StringColumn,
}

impl SerializedKeysColumnIter {
    pub fn create(column: &StringColumn) -> Result<SerializedKeysColumnIter> {
        Ok(SerializedKeysColumnIter {
            column: column.clone(),
        })
    }
}

impl KeysColumnIter<[u8]> for SerializedKeysColumnIter {
    type Iterator<'a> = StringIterator<'a> where Self: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.column.iter()
    }
}
