use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NobKind {
    NobRootKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NobRootKind {
    NobRootKind0,
}
