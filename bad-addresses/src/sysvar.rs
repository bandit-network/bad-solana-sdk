pub use badchain_sdk_ids::sysvar::slot_hashes::{check_id, id, ID};
use {crate::BadAddresses, badchain_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(BadAddresses);
