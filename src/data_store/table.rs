use crate::Identifiable;

use super::DataStoreError;

pub trait Table {
    type Record: Identifiable;

    fn name(&self) -> &'static str;

    const LIST_DEFAULT: usize = 20;
    const LIST_MAX: usize = 100;
    // !? assert!(1 <= Self::LIST_DEFAULT && Self::LIST_DEFAULT <= Self::LIST_MAX);

    fn list<Ids: IntoIterator<Item = <Self::Record as Identifiable>::Id>>(
        &self,
        ids: Ids,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Self::Record>, DataStoreError<Self::Record>>;

    fn list_mut<Ids: IntoIterator<Item = <Self::Record as Identifiable>::Id>>(
        &mut self,
        ids: Ids,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Self::Record>, DataStoreError<Self::Record>> {
        self.list(ids, limit, offset)
    }

    fn create(
        &mut self,
        record: Self::Record,
    ) -> Result<Self::Record, DataStoreError<Self::Record>>;

    fn read(
        &self,
        id: <Self::Record as Identifiable>::Id,
    ) -> Result<Option<Self::Record>, DataStoreError<Self::Record>>;

    fn read_mut(
        &mut self,
        id: <Self::Record as Identifiable>::Id,
    ) -> Result<Option<Self::Record>, DataStoreError<Self::Record>> {
        self.read(id)
    }

    fn update(&mut self, record: Self::Record) -> Result<(), DataStoreError<Self::Record>>;

    fn delete(
        &mut self,
        id: <Self::Record as Identifiable>::Id,
    ) -> Result<(), DataStoreError<Self::Record>>;

    fn truncate(&mut self) -> Result<(), DataStoreError<Self::Record>>;
}
