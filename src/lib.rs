pub mod transport;

use std::{fmt::Display, hash::Hash};

pub trait Identifiable {
    type Id: Default + Display + Eq + Hash + Copy;
    fn id(&self) -> Self::Id;
}
