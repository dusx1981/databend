// Copyright 2023 Datafuse Labs.
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

use common_exception::Result;
use common_expression::DataBlock;
use common_expression::TableSchemaRef;
use opendal::Operator;
use storages_common_blocks::blocks_to_parquet;
use storages_common_table_meta::table::TableCompression;
use uuid::Uuid;

pub(super) struct ResultCacheWriter {
    operator: Operator,
    location: String,

    current_bytes: usize,
    max_bytes: usize,
    num_rows: usize,

    schema: TableSchemaRef,
    blocks: Vec<DataBlock>,
}

impl ResultCacheWriter {
    pub fn create(
        schema: TableSchemaRef,
        location: String,
        operator: Operator,
        max_bytes: usize,
    ) -> Self {
        ResultCacheWriter {
            location,
            operator,
            current_bytes: 0,
            max_bytes,
            num_rows: 0,
            schema,
            blocks: vec![],
        }
    }

    pub fn append_block(&mut self, block: DataBlock) {
        self.current_bytes += block.memory_size();
        self.num_rows += block.num_rows();
        self.blocks.push(block);
    }

    pub fn over_limit(&self) -> bool {
        self.current_bytes > self.max_bytes
    }

    /// Write the result cache to the storage and return the location.
    #[async_backtrace::framed]
    pub async fn write_to_storage(&self) -> Result<String> {
        let mut buf = Vec::with_capacity(self.current_bytes);
        let _ = blocks_to_parquet(
            &self.schema,
            self.blocks.clone(),
            &mut buf,
            TableCompression::None,
        )?;

        let file_location = format!("{}/{}.parquet", self.location, Uuid::new_v4().as_simple());

        self.operator.write(&file_location, buf).await?;
        Ok(file_location)
    }

    pub fn current_bytes(&self) -> usize {
        self.current_bytes
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
}
