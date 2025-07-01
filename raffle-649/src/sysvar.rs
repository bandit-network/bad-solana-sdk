pub use badchain_sdk_ids::sysvar::raffle_649::{check_id, id};
use {crate::Raffle, solana_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Raffle);
