use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

use uuid::Uuid;

use crate::{
    data_store::{DataStoreError, Table as CRUDTable},
    Identifiable,
};

use super::{AutoIncrement, UUIDv4};

pub struct Table<
    Record: Identifiable + Clone,
    Generator: Iterator<Item = Record::Id>,
    Store = BTreeMap<<Record as Identifiable>::Id, Record>,
> {
    name: &'static str,
    generator: Generator,
    store: Store,
    _phantom_record: PhantomData<Record>,
}

impl<Record: Identifiable + Clone, Generator: Iterator<Item = Record::Id>>
    Table<Record, Generator>
{
    pub fn new(name: &'static str, generator: Generator) -> Self {
        Self {
            name,
            generator,
            store: BTreeMap::new(),
            _phantom_record: PhantomData,
        }
    }
}

impl<Record: Identifiable<Id = i8> + Clone> Table<Record, AutoIncrement<i8>> {
    pub fn auto_incremented(name: &'static str, from: i8) -> Self {
        Self::new(name, AutoIncrement::from(from.max(0)))
    }
}

impl<Record: Identifiable<Id = i16> + Clone> Table<Record, AutoIncrement<i16>> {
    pub fn auto_incremented(name: &'static str, from: i16) -> Self {
        Self::new(name, AutoIncrement::from(from.max(0)))
    }
}

impl<Record: Identifiable<Id = i32> + Clone> Table<Record, AutoIncrement<i32>> {
    pub fn auto_incremented(name: &'static str, from: i32) -> Self {
        Self::new(name, AutoIncrement::from(from.max(0)))
    }
}

impl<Record: Identifiable<Id = i64> + Clone> Table<Record, AutoIncrement<i64>> {
    pub fn auto_incremented(name: &'static str, from: i64) -> Self {
        Self::new(name, AutoIncrement::from(from.max(0)))
    }
}

impl<Record: Identifiable<Id = i128> + Clone> Table<Record, AutoIncrement<i128>> {
    pub fn auto_incremented(name: &'static str, from: i128) -> Self {
        Self::new(name, AutoIncrement::from(from.max(0)))
    }
}

impl<Record: Identifiable<Id = isize> + Clone> Table<Record, AutoIncrement<isize>> {
    pub fn auto_incremented(name: &'static str, from: isize) -> Self {
        Self::new(name, AutoIncrement::from(from.max(0)))
    }
}

impl<Record: Identifiable<Id = u8> + Clone> Table<Record, AutoIncrement<u8>> {
    pub fn auto_incremented(name: &'static str, from: u8) -> Self {
        Self::new(name, AutoIncrement::from(from))
    }
}

impl<Record: Identifiable<Id = u16> + Clone> Table<Record, AutoIncrement<u16>> {
    pub fn auto_incremented(name: &'static str, from: u16) -> Self {
        Self::new(name, AutoIncrement::from(from))
    }
}

impl<Record: Identifiable<Id = u32> + Clone> Table<Record, AutoIncrement<u32>> {
    pub fn auto_incremented(name: &'static str, from: u32) -> Self {
        Self::new(name, AutoIncrement::from(from))
    }
}

impl<Record: Identifiable<Id = u64> + Clone> Table<Record, AutoIncrement<u64>> {
    pub fn auto_incremented(name: &'static str, from: u64) -> Self {
        Self::new(name, AutoIncrement::from(from))
    }
}

impl<Record: Identifiable<Id = u128> + Clone> Table<Record, AutoIncrement<u128>> {
    pub fn auto_incremented(name: &'static str, from: u128) -> Self {
        Self::new(name, AutoIncrement::from(from))
    }
}

impl<Record: Identifiable<Id = usize> + Clone> Table<Record, AutoIncrement<usize>> {
    pub fn auto_incremented(name: &'static str, from: usize) -> Self {
        Self::new(name, AutoIncrement::from(from))
    }
}

impl<Record: Identifiable<Id = Uuid> + Clone>
    Table<Record, UUIDv4<Uuid>, HashMap<Record::Id, Record>>
{
    pub fn uuid_v4(name: &'static str) -> Self {
        Self {
            name,
            generator: UUIDv4::new(),
            store: HashMap::new(),
            _phantom_record: PhantomData,
        }
    }
}

impl<Record: Identifiable<Id = String> + Clone>
    Table<Record, UUIDv4<String>, HashMap<Record::Id, Record>>
{
    pub fn uuid_v4(name: &'static str) -> Self {
        Self {
            name,
            generator: UUIDv4::new(),
            store: HashMap::new(),
            _phantom_record: PhantomData,
        }
    }
}

impl<Record: Identifiable<Id = u128> + Clone>
    Table<Record, UUIDv4<u128>, HashMap<Record::Id, Record>>
{
    pub fn uuid_v4(name: &'static str) -> Self {
        Self {
            name,
            generator: UUIDv4::new(),
            store: HashMap::new(),
            _phantom_record: PhantomData,
        }
    }
}

impl<Record: Identifiable + Clone, Generator: Iterator<Item = Record::Id>> CRUDTable
    for Table<Record, Generator>
{
    type Record = Record;

    fn name(&self) -> &'static str {
        self.name
    }

    fn list<Ids: IntoIterator<Item = Record::Id>>(
        &self,
        ids: Ids,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Record>, DataStoreError<Record>> {
        let offset = offset.unwrap_or(0);
        let limit = limit.unwrap_or(Self::LIST_DEFAULT).clamp(1, Self::LIST_MAX);
        let records = (ids.into_iter())
            .skip(offset)
            .take(limit)
            .filter_map(|id| self.store.get(&id).cloned())
            .fold(Vec::with_capacity(limit), |mut acc, record| {
                acc.push(record);
                acc
            });
        Ok(records)
    }

    fn create(&mut self, mut record: Record) -> Result<Record, DataStoreError<Record>> {
        match self.generator.next() {
            None => Err(DataStoreError::GeneratorExhausted { table: self.name }),
            Some(id) => {
                record.set_id(id);
                self.store.insert(id, record.clone());
                Ok(record)
            }
        }
    }

    fn read(&self, id: Record::Id) -> Result<Option<Record>, DataStoreError<Record>> {
        Ok(self.store.get(&id).cloned())
    }

    fn update(&mut self, record: Record) -> Result<(), DataStoreError<Record>> {
        self.store.entry(record.id()).and_modify(|r| *r = record);
        Ok(())
    }

    fn delete(&mut self, id: Record::Id) -> Result<(), DataStoreError<Record>> {
        self.store.remove(&id);
        Ok(())
    }

    fn truncate(&mut self) -> Result<(), DataStoreError<Record>> {
        self.store.clear();
        Ok(())
    }
}
