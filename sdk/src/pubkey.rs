#[cfg(feature = "full")]
pub use badchain_pubkey::new_rand;
#[cfg(target_os = "solana")]
pub use badchain_pubkey::syscalls;
pub use badchain_pubkey::{
    bytes_are_curve_point, ParsePubkeyError, Pubkey, PubkeyError, MAX_SEEDS, MAX_SEED_LEN,
    PUBKEY_BYTES,
};
