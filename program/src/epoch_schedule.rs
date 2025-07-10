#[deprecated(
    since = "2.1.0",
    note = "Use solana-clock and solana-epoch-schedule crates instead."
)]
pub use {
    badchain_clock::{Epoch, Slot, DEFAULT_SLOTS_PER_EPOCH},
    badchain_epoch_schedule::*,
};
