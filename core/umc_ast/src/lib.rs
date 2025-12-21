use std::{fmt::Debug, hash::Hash};

pub trait Ast: Sized + Debug + Clone + PartialEq + Eq + Hash {}
