mod id;
mod kind;

use id::NobID;
use kind::{NobKind, NobRootKind};

use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type NobResultSuccess = String;
pub type NobResultError = String;
pub type NobResultErrorCode = i32;

const NOB_NAME_DEFAULT: &str = "Unnamed";

pub fn main(app: &dyn NobApp) -> Result<NobResultSuccess, (NobResultError, NobResultErrorCode)> {
    app.main()
}

pub trait NobApp {
    fn main(&self) -> Result<NobResultSuccess, (NobResultError, NobResultErrorCode)> {
        Err((
            "You need to implement `NobApp::main()`, otherwise \
you will be seeing this error message right now, and nothing \
useful is going to happen."
                .to_string(),
            1, // NobResultErrorCode
        ))
    }
}

pub trait NobObj {}

#[derive(Serialize, Deserialize, Debug)]
pub struct NobRoot {
    /** A global enum identifier for the type of Nob object.

    Every type of Nob object has one to differentiate
    it from the other types. */
    pub(crate) global_id: NobID,

    /** A hierarchial enum identifier to identify the Nob object's
    parent type. */
    pub(crate) parent_kind: NobKind,

    /** A hierarchial enum identifier for the type of Nob object.

    Helps to track where this type of object belongs in a typical
    Nob stack, when considered along with its `parent_kind`. */
    pub(crate) kind: NobRootKind,

    /** A short name for the Nob object instance. It doesn't need
    to be unique.

    You can pass the desired name to set here as an argument to
    the `new(name: Option<&str>) -> Self` function when
    instantiating an object of this type if you want, otherwise it
    will be set to some value automatically. */
    pub(crate) name: String,

    /** A descriptive name for the Nob object instance. It will be
    auto-generated based on some of the other field values in
    the object. It won't necessarily be unique for each object
    instance. */
    pub(crate) full_name: String,
}

impl NobRoot {
    pub fn new(name: Option<&str>) -> Self {
        const GLOBAL_ID: NobID = NobID::NobRoot;
        const PARENT_KIND: NobKind = NobKind::NobRootKind;
        const KIND: NobRootKind = NobRootKind::NobRootKind0;

        let name = name.unwrap_or(NOB_NAME_DEFAULT);
        let full_name = format!("[{:?}::{:?}::{:?}::{}]", GLOBAL_ID, PARENT_KIND, KIND, name);

        println!("New object created: {}", &full_name);

        Self {
            global_id: GLOBAL_ID,
            parent_kind: PARENT_KIND,
            kind: KIND,
            name: name.to_string(),
            full_name,
        }
    }
}

impl NobApp for NobRoot {
    fn main(&self) -> Result<NobResultSuccess, (NobResultError, NobResultErrorCode)> {
        Ok(format!("{} says goodbye.", self.name))
    }
}

impl NobObj for NobRoot {}

impl Display for NobRoot {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
