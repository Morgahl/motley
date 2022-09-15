use thiserror::Error;

use crate::Identifiable;

#[derive(Error, Debug)]
pub enum DataStoreError<Record: Identifiable> {
    #[error("Record with id {id} already exists in table {table}")]
    RecordNotUnique { id: Record::Id, table: &'static str },
    #[error("Record with id {id} does not exist in table {table}")]
    RecordNotFound { id: Record::Id, table: &'static str },
    #[error("Generator exhausted for table {table}")]
    GeneratorExhausted { table: &'static str },
}
