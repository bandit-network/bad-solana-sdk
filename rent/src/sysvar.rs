pub use badchain_sdk_ids::sysvar::rent::{check_id, id, ID};
use {crate::Rent, badchain_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Rent);
