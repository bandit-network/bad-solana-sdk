//! Information about the network's clock, ticks, slots, etc.
//!
//! Time in Solana is marked primarily by _slots_, which occur approximately every
//! 400 milliseconds, and are numbered sequentially. For every slot, a leader is
//! chosen from the validator set, and that leader is expected to produce a new
//! block, though sometimes leaders may fail to do so. Blocks can be identified
//! by their slot number, and some slots do not contain a block.
//!
//! An approximation of the passage of real-world time can be calculated by
//! multiplying a number of slots by [`DEFAULT_MS_PER_SLOT`], which is a constant target
//! time for the network to produce slots. Note though that this method suffers
//! a variable amount of drift, as the network does not produce slots at exactly
//! the target rate, and the greater number of slots being calculated for, the
//! greater the drift. Epochs cannot be used this way as they contain variable
//! numbers of slots.
//!
//! The network's current view of the real-world time can always be accessed via
//! [`Clock::unix_timestamp`], which is produced by an [oracle derived from the
//! validator set][oracle].
//!
//! [oracle]: https://docs.solanalabs.com/implemented-proposals/validator-timestamp-oracle
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod sysvar;

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
use solana_clock::Slot;
use solana_sdk_macro::CloneZeroed;

pub type RoundId = u64;

pub type DrawSlot = Slot;

pub type WinningNumber = [u8; 6];

pub type BonusNumber = u8;

pub type Entropy = [u8; 32];

pub type TotalRewardPool = u64;

pub type WinnerCounts = [u32; 6];

pub type TierRatios = [u16; 6];

/// A representation of raffle.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, CloneZeroed, Default, PartialEq, Eq)]
pub struct Raffle {
    pub current_round: RoundId,
    pub draw_slot: DrawSlot,
    pub winning_numbers: WinningNumber,
    pub bonus_number: BonusNumber,
    pub entropy: Entropy,
    pub total_reward_pool: TotalRewardPool,
    pub winner_counts: WinnerCounts,
    pub tier_ratios: TierRatios,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let clock = Raffle {
            ..Default::default()
        };
        let cloned_clock = clock.clone();
        assert_eq!(cloned_clock, clock);
    }
}
