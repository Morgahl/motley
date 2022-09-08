pub mod mock;

use anyhow::Result;

use crate::Identifiable;

pub trait Table<Record: Identifiable> {
    fn create(&mut self, record: Record) -> Result<Record::Id>;
    fn read(&self, id: Record::Id) -> Result<Option<&Record>>;
    fn list<Ids: IntoIterator<Item = Record::Id>>(&self, ids: Ids) -> Result<Vec<&Record>>;
    fn update(&mut self, record: Record) -> Result<Option<Record::Id>>;
    fn delete(&mut self, id: Record::Id) -> Result<()>;
}
