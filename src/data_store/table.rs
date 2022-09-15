use crate::Identifiable;

use super::DataStoreError;

pub trait Table {
    type Record: Identifiable;

    type Generator: Iterator<Item = <Self::Record as Identifiable>::Id>;

    fn name(&self) -> &'static str;

    fn list<Ids: IntoIterator<Item = <Self::Record as Identifiable>::Id>>(
        &self,
        ids: Ids,
    ) -> Result<Vec<Self::Record>, DataStoreError<Self::Record>>;

    fn create(
        &mut self,
        record: Self::Record,
    ) -> Result<Self::Record, DataStoreError<Self::Record>>;

    fn read(
        &self,
        id: <Self::Record as Identifiable>::Id,
    ) -> Result<Option<Self::Record>, DataStoreError<Self::Record>>;

    fn update(&mut self, record: Self::Record) -> Result<(), DataStoreError<Self::Record>>;

    fn delete(
        &mut self,
        id: <Self::Record as Identifiable>::Id,
    ) -> Result<(), DataStoreError<Self::Record>>;

    fn truncate(&mut self) -> Result<(), DataStoreError<Self::Record>>;
}
