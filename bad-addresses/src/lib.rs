//! A type to hold data for the [`SlotHashes` sysvar][sv].
//!
//! [sv]: https://docs.solanalabs.com/runtime/sysvars#slothashes
//!
//! The sysvar ID is declared in [`solana_program::sysvar::slot_hashes`].
//!
//! [`solana_program::sysvar::slot_hashes`]: https://docs.rs/solana-program/latest/solana_program/sysvar/slot_hashes/index.html

#[cfg(feature = "sysvar")]
pub mod sysvar;

use rand::seq::SliceRandom;
use rand::thread_rng;

use {
    badchain_pubkey::Pubkey,
    std::{
        iter::FromIterator,
        ops::Deref,
        sync::atomic::{AtomicUsize, Ordering},
    },
};

pub const MAX_ENTRIES: usize = 512; // about 2.5 minutes to get your vote in

// This is to allow tests with custom slot hash expiry to avoid having to generate
// 512 blocks for such tests.
static NUM_ENTRIES: AtomicUsize = AtomicUsize::new(MAX_ENTRIES);

pub fn get_entries() -> usize {
    NUM_ENTRIES.load(Ordering::Relaxed)
}

pub fn set_entries_for_tests_only(entries: usize) {
    NUM_ENTRIES.store(entries, Ordering::Relaxed);
}

pub type BadAddress = Pubkey;

#[repr(C)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
#[derive(PartialEq, Eq, Debug, Default)]
pub struct BadAddresses(Vec<BadAddress>);

impl BadAddresses {
    pub fn add(&mut self, pubkey: Pubkey) {
        match self.binary_search_by(|probe| pubkey.cmp(probe)) {
            Ok(index) => (self.0)[index] = pubkey,
            Err(index) => (self.0).insert(index, pubkey),
        }
        (self.0).truncate(get_entries());
    }

    pub fn position(&self, pubkey: Pubkey) -> Option<usize> {
        self.binary_search_by(|probe| pubkey.cmp(probe)).ok()
    }

    // Returns a random address from array.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn get(&self) -> Option<&Pubkey> {
        self.0.choose(&mut thread_rng())
    }

    pub fn new(bad_addresses: &[BadAddress]) -> Self {
        let slot_hashes = bad_addresses.to_vec();
        Self(slot_hashes)
    }

    pub fn bad_addresses(&self) -> &[BadAddress] {
        &self.0
    }
}

impl FromIterator<Pubkey> for BadAddresses {
    fn from_iter<I: IntoIterator<Item = Pubkey>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Deref for BadAddresses {
    type Target = Vec<BadAddress>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (one, two, three) = (
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        );
        let mut bad_addresses = BadAddresses::new(&[two, one]);
        bad_addresses.add(three);
        assert_eq!(bad_addresses, BadAddresses(vec![three, two, one]));

        let mut bad_addresses = BadAddresses::new(&[]);
        for _ in 0..MAX_ENTRIES + 1 {
            bad_addresses.add(Pubkey::new_unique());
        }
        assert_eq!(bad_addresses.len(), MAX_ENTRIES);
    }
}
