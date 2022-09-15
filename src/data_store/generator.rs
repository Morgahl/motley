use std::{marker::PhantomData, ops::Add};

use uuid::Uuid;

use crate::Identifier;

pub struct UUIDv4<T> {
    _phantom_t: PhantomData<T>,
}

impl<T> UUIDv4<T> {
    pub fn new() -> Self {
        Self {
            _phantom_t: PhantomData,
        }
    }
}

impl Iterator for UUIDv4<Uuid> {
    type Item = Uuid;

    fn next(&mut self) -> Option<Self::Item> {
        Some(uuid::Uuid::new_v4())
    }
}

impl Iterator for UUIDv4<String> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(uuid::Uuid::new_v4().to_string())
    }
}

impl Iterator for UUIDv4<u128> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        Some(uuid::Uuid::new_v4().as_u128())
    }
}

pub struct AutoIncrement<Idx: Identifier + Add<Idx>>(Idx);

impl<Idx: Identifier + Add<Idx>> AutoIncrement<Idx> {
    pub fn from(from: Idx) -> Self {
        Self(from)
    }
}

impl Iterator for AutoIncrement<i8> {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<i16> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<i64> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<i128> {
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<isize> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<u8> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<u16> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<u64> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<u128> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}

impl Iterator for AutoIncrement<usize> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.0;
        self.0 = self.0 + 1;
        Some(id)
    }
}
