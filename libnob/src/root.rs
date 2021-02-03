/** Copyright (c) 2021 Jeremy Carter <jeremy@jeremycarter.ca>

    All uses of this project in part or in whole are governed
    by the terms of the license contained in the file titled
    "LICENSE" that's distributed along with the project, which
    can be found in the top-level directory of this project.

    If you don't agree to follow those terms or you won't
    follow them, you are not allowed to use this project or
    anything that's made with parts of it at all. The project
    is also	depending on some third-party technologies, and
    some of those are governed by their own separate licenses,
    so furthermore, whenever legally possible, all license
    terms from all of the different technologies apply, with
    this project's license terms taking first priority.
*/
use super::*;
use id::NobID;
use kind::{NobKind, NobRootKind};
use obj::NobObj;
use NOB_NAME_DEFAULT;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

    /** A descriptive identifier for the Nob object instance.
    It will be auto-generated based on some of the other field
    values in the object. It won't necessarily be unique for each
    object instance. It will be used later for scripting purposes. */
    pub(crate) tag: String,
}

impl NobRoot {
    pub fn new(name: Option<&str>) -> Self {
        const GLOBAL_ID: NobID = NobID::NobRoot;
        const PARENT_KIND: NobKind = NobKind::NobRootKind;
        const KIND: NobRootKind = NobRootKind::NobRootKind0;

        let name = name.unwrap_or(NOB_NAME_DEFAULT);
        let tag = format!("[{:?}::{:?}::{:?}::{}]", GLOBAL_ID, PARENT_KIND, KIND, name);

        println!("New object created: {}", &tag);

        Self {
            global_id: GLOBAL_ID,
            parent_kind: PARENT_KIND,
            kind: KIND,
            name: name.to_string(),
            tag,
        }
    }
}

impl NobApp for NobRoot {
    fn main(&self) -> Result<NobResultSuccess, (NobResultError, NobResultErrorCode)> {
        Ok(format!("{} says goodbye.", &self.name))
    }
}

impl NobObj for NobRoot {}

impl Display for NobRoot {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", &serde_json::to_string_pretty(self).unwrap())
    }
}
