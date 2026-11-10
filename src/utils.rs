use serde::{Serialize, Deserialize};

#[derive(Clone, Eq, PartialEq, Hash, Copy, Serialize, Deserialize)]
pub struct Id(pub u64);
