use jiff::Zoned;
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

::pgrx::pg_module_magic!();

#[derive(
    Serialize,
    Deserialize,
    PostgresType,
    PostgresEq,
    PostgresOrd,
    PostgresHash,
    Eq,
    Ord,
    Hash,
    PartialEq,
    PartialOrd,
)]
#[inoutfuncs]
struct ZonedDateTime(Zoned);

impl InOutFuncs for ZonedDateTime {
    fn input(input: &core::ffi::CStr) -> Self
    where
        Self: Sized,
    {
        let input = input.to_str().unwrap().to_owned();
        let zdt: Zoned = input.parse().unwrap();
        ZonedDateTime(zdt)
    }

    fn output(&self, buffer: &mut pgrx::StringInfo) {
        buffer.push_str(&self.0.to_string())
    }
}

#[pg_extern]
fn zdt_now() -> ZonedDateTime {
    ZonedDateTime(Zoned::now())
}
