use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::{Identifiable, IdentifiableGenerator};

use super::Table as CRUDTable;

pub struct Table<Record: Identifiable, Generator: IdentifiableGenerator<Record>>(
    HashMap<Record::Id, Record>,
    Generator,
);

impl<Record: Identifiable, Generator: IdentifiableGenerator<Record>> Table<Record, Generator> {
    pub fn new(gen: Generator) -> Self {
        Self(HashMap::new(), gen)
    }
}

impl<Record: Identifiable, Generator: IdentifiableGenerator<Record>> CRUDTable<Record>
    for Table<Record, Generator>
{
    fn create(&mut self, mut record: Record) -> Result<Record::Id> {
        match self.0.contains_key(&record.id()) {
            true => bail!("Record already exists"),
            false => match self.1.generate_id() {
                Some(id) => {
                    record.set_id(id);
                    self.0.insert(id, record);
                    Ok(id)
                }
                None => bail!("Failed to generate id"),
            },
        }
    }

    fn read(&self, id: Record::Id) -> Result<Option<&Record>> {
        Ok(self.0.get(&id))
    }

    fn list<Ids: IntoIterator<Item = Record::Id>>(&self, ids: Ids) -> Result<Vec<&Record>> {
        Ok(ids
            .into_iter()
            .filter_map(|id| self.0.get(&id))
            .collect::<Vec<&Record>>())
    }

    fn update(&mut self, record: Record) -> Result<Option<Record::Id>> {
        let id = record.id();
        match self.0.contains_key(&id) {
            false => Ok(None),
            true => Ok(Some(
                *self.0.entry(id).and_modify(|entry| *entry = record).key(),
            )),
        }
    }

    fn delete(&mut self, id: Record::Id) -> Result<()> {
        self.0.remove(&id);
        Ok(())
    }
}
