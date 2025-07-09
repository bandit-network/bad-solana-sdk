pub use badchain_sdk_ids::sysvar::clock::{check_id, id, ID};
use {crate::Clock, badchain_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Clock);
