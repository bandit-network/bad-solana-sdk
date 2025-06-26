//! The most recent hashes of a slot's parent banks.
//!
//! The _slot hashes sysvar_ provides access to the [`SlotHashes`] type.
//!
//! The [`Sysvar::from_account_info`] and [`Sysvar::get`] methods always return
//! [`solana_program_error::ProgramError::UnsupportedSysvar`] because this sysvar account is too large
//! to process on-chain. Thus this sysvar cannot be accessed on chain, though
//! one can still use the [`SysvarId::id`], [`SysvarId::check_id`] and
//! [`Sysvar::size_of`] methods in an on-chain program, and it can be accessed
//! off-chain through RPC.
//!
//! [`SysvarId::id`]: https://docs.rs/solana-sysvar-id/latest/solana_sysvar_id/trait.SysvarId.html#tymethod.id
//! [`SysvarId::check_id`]: https://docs.rs/solana-sysvar-id/latest/solana_sysvar_id/trait.SysvarId.html#tymethod.check_id
//!
//! # Examples
//!
//! Calling via the RPC client:
//!
//! ```
//! # use solana_program::example_mocks::solana_sdk;
//! # use solana_program::example_mocks::solana_rpc_client;
//! # use solana_sdk::account::Account;
//! # use solana_rpc_client::rpc_client::RpcClient;
//! # use solana_sdk_ids::sysvar::slot_hashes;
//! # use solana_slot_hashes::SlotHashes;
//! # use anyhow::Result;
//! #
//! fn print_sysvar_slot_hashes(client: &RpcClient) -> Result<()> {
//! #   client.set_get_account_response(slot_hashes::ID, Account {
//! #       lamports: 1009200,
//! #       data: vec![1, 0, 0, 0, 0, 0, 0, 0, 86, 190, 235, 7, 0, 0, 0, 0, 133, 242, 94, 158, 223, 253, 207, 184, 227, 194, 235, 27, 176, 98, 73, 3, 175, 201, 224, 111, 21, 65, 73, 27, 137, 73, 229, 19, 255, 192, 193, 126],
//! #       owner: solana_sdk_ids::system_program::ID,
//! #       executable: false,
//! #       rent_epoch: 307,
//! # });
//! #
//!     let slot_hashes = client.get_account(&slot_hashes::ID)?;
//!     let data: SlotHashes = bincode::deserialize(&slot_hashes.data)?;
//!
//!     Ok(())
//! }
//! #
//! # let client = RpcClient::new(String::new());
//! # print_sysvar_slot_hashes(&client)?;
//! #
//! # Ok::<(), anyhow::Error>(())
//! ```

use badchain_bad_addresses::BadAddress;
#[cfg(feature = "bytemuck")]
use bytemuck_derive::{Pod, Zeroable};
#[cfg(feature = "bytemuck")]
use solana_pubkey::Pubkey;
#[cfg(feature = "bincode")]
use {crate::Sysvar, solana_account_info::AccountInfo};

#[cfg(feature = "bytemuck")]
const U64_SIZE: usize = std::mem::size_of::<u64>();

#[cfg(any(feature = "bytemuck", feature = "bincode"))]
const SYSVAR_LEN: usize = 16_392; // golden, update if MAX_ENTRIES changes

pub use {
    badchain_bad_addresses::BadAddresses,
    solana_sdk_ids::sysvar::bad_addresses::{check_id, id, ID},
    solana_sysvar_id::SysvarId,
};

#[cfg(feature = "bincode")]
impl Sysvar for BadAddresses {
    // overridex
    fn size_of() -> usize {
        // hard-coded so that we don't have to construct an empty
        SYSVAR_LEN
    }
    fn from_account_info(
        _account_info: &AccountInfo,
    ) -> Result<Self, solana_program_error::ProgramError> {
        // This sysvar is too large to bincode::deserialize in-program
        Err(solana_program_error::ProgramError::UnsupportedSysvar)
    }
}

/// A bytemuck-compatible (plain old data) version of `BadAddress`.
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct PodBadAddress {
    pub address: BadAddress,
}

#[cfg(feature = "bytemuck")]
/// API for querying of the `SlotHashes` sysvar by on-chain programs.
///
/// Hangs onto the allocated raw buffer from the account data, which can be
/// queried or accessed directly as a slice of `PodSlotHash`.
#[derive(Default)]
pub struct PodBadAddresses {
    data: Vec<u8>,
    bad_addresses_start: usize,
    bad_addresses_end: usize,
}

#[cfg(feature = "bytemuck")]
impl PodBadAddresses {
    /// Fetch all of the raw sysvar data using the `sol_get_sysvar` syscall.
    pub fn fetch() -> Result<Self, solana_program_error::ProgramError> {
        // Allocate an uninitialized buffer for the raw sysvar data.
        let sysvar_len = SYSVAR_LEN;
        let mut data = vec![0; sysvar_len];

        // Ensure the created buffer is aligned to 8.
        if data.as_ptr().align_offset(8) != 0 {
            return Err(solana_program_error::ProgramError::InvalidAccountData);
        }

        // Populate the buffer by fetching all sysvar data using the
        // `sol_get_sysvar` syscall.
        crate::get_sysvar(
            &mut data,
            &BadAddresses::id(),
            /* offset */ 0,
            /* length */ sysvar_len as u64,
        )?;

        // Get the number of slot hashes present in the data by reading the
        // `u64` length at the beginning of the data, then use that count to
        // calculate the length of the slot hashes data.
        //
        // The rest of the buffer is uninitialized and should not be accessed.
        let length = data
            .get(..U64_SIZE)
            .and_then(|bytes| bytes.try_into().ok())
            .map(u64::from_le_bytes)
            .and_then(|length| length.checked_mul(std::mem::size_of::<PodBadAddress>() as u64))
            .ok_or(solana_program_error::ProgramError::InvalidAccountData)?;

        let bad_addresses_start = U64_SIZE;
        let bad_addresses_end = bad_addresses_start.saturating_add(length as usize);

        Ok(Self {
            data,
            bad_addresses_start,
            bad_addresses_end,
        })
    }

    /// Return the `SlotHashes` sysvar data as a slice of `PodSlotHash`.
    /// Returns a slice of only the initialized sysvar data.
    pub fn as_slice(&self) -> Result<&[PodBadAddress], solana_program_error::ProgramError> {
        self.data
            .get(self.bad_addresses_start..self.bad_addresses_end)
            .and_then(|data| bytemuck::try_cast_slice(data).ok())
            .ok_or(solana_program_error::ProgramError::InvalidAccountData)
    }

    /// Given a slot, get its corresponding hash in the `SlotHashes` sysvar
    /// data. Returns `None` if the slot is not found.
    // pub fn get(&self) -> Result<Option<Hash>, solana_program_error::ProgramError> {
    //     self.as_slice().map(|pod_hashes| {
    //         pod_hashes
    //             .binary_search_by(|PodBadAddress { slot: this, .. }| slot.cmp(this))
    //             .map(|idx| pod_hashes[idx].hash)
    //             .ok()
    //     });
    // }

    pub fn get(&self) -> Result<Option<Pubkey>, solana_program_error::ProgramError> {
        use rand::{seq::SliceRandom, thread_rng};
        self.as_slice()
            .map(|pod_address| Some(pod_address.choose(&mut thread_rng()).unwrap().address))
    }

    // TODO(theja) : review about needing of this
    // Given a slot, get its position in the `SlotHashes` sysvar data. Returns
    // `None` if the slot is not found.
    // pub fn position(
    //     &self,
    //     slot: &Slot,
    // ) -> Result<Option<usize>, solana_program_error::ProgramError> {
    //     self.as_slice().map(|pod_hashes| {
    //         pod_hashes
    //             .binary_search_by(|PodSlotHash { slot: this, .. }| slot.cmp(this))
    //             .ok()
    //     })
    // }
}

/// API for querying the `SlotHashes` sysvar.
#[deprecated(since = "2.1.0", note = "Please use `PodSlotHashes` instead")]
pub struct BadAddressesSysvar;

#[allow(deprecated)]
impl BadAddressesSysvar {
    #[cfg(feature = "bytemuck")]
    /// Get a value from the sysvar entries by its key.
    /// Returns `None` if the key is not found.
    pub fn get() -> Result<Option<Pubkey>, solana_program_error::ProgramError> {
        get_pod_bad_addresses().map(|pod_hashes| {
            use rand::{seq::SliceRandom, thread_rng};
            pod_hashes
                .choose(&mut thread_rng())
                .map(|selected| selected.address)
        })
    }

    // TODO(theja) : review about needing of this
    // #[cfg(feature = "bytemuck")]
    // /// Get the position of an entry in the sysvar by its key.
    // /// Returns `None` if the key is not found.
    // pub fn position(slot: &Slot) -> Result<Option<usize>, solana_program_error::ProgramError> {
    //     get_pod_bad_addresses().map(|pod_hashes| {
    //         pod_hashes
    //             .binary_search_by(|PodSlotHash { slot: this, .. }| slot.cmp(this))
    //             .ok()
    //     })
    // }
}

#[cfg(feature = "bytemuck")]
fn get_pod_bad_addresses() -> Result<Vec<PodBadAddress>, solana_program_error::ProgramError> {
    let mut pod_addresses = vec![PodBadAddress::default(); badchain_bad_addresses::MAX_ENTRIES];
    {
        let data = bytemuck::try_cast_slice_mut::<PodBadAddress, u8>(&mut pod_addresses)
            .map_err(|_| solana_program_error::ProgramError::InvalidAccountData)?;

        // Ensure the created buffer is aligned to 8.
        if data.as_ptr().align_offset(8) != 0 {
            return Err(solana_program_error::ProgramError::InvalidAccountData);
        }

        let offset = 8; // Vector length as `u64`.
        let length = (SYSVAR_LEN as u64).saturating_sub(offset);
        crate::get_sysvar(data, &BadAddresses::id(), offset, length)?;
    }
    Ok(pod_addresses)
}

#[cfg(test)]
mod tests {
    use {
        super::*, crate::tests::mock_get_sysvar_syscall, serial_test::serial,
        solana_slot_hashes::MAX_ENTRIES, test_case::test_case,
    };

    #[test]
    fn test_size_of() {
        assert_eq!(
            BadAddresses::size_of(),
            bincode::serialized_size(
                &(0..MAX_ENTRIES)
                    .map(|_| (Pubkey::new_unique()))
                    .collect::<BadAddresses>()
            )
            .unwrap() as usize
        );
    }

    #[test_case(64)]
    #[test_case(128)]
    #[test_case(192)]
    #[test_case(256)]
    #[test_case(384)]
    #[test_case(MAX_ENTRIES)]
    #[serial]
    fn test_random_get(num_entries: usize) {
        let mut bad_addresses = vec![];
        for _ in 0..num_entries {
            bad_addresses.push(Pubkey::new_unique());
        }

        let check_bad_addresses = BadAddresses::new(&bad_addresses);
        mock_bad_addresses(&check_bad_addresses);

        let pod_bad_addresses = PodBadAddresses::fetch().unwrap();

        let addr1_res = pod_bad_addresses.get();
        let addr2_res = pod_bad_addresses.get();

        assert_ne!(addr1_res.is_ok(), false);
        assert_ne!(addr2_res.is_ok(), false);

        let addr1 = addr1_res.unwrap();
        let addr2 = addr2_res.unwrap();

        // none testing
        assert_ne!(addr1, None);
        assert_ne!(addr2, None);

        // check random address
        assert_ne!(addr1.unwrap(), addr2.unwrap());
    }

    fn mock_bad_addresses(slot_hashes: &BadAddresses) {
        // The data is always `SlotHashes::size_of()`.
        let mut data = vec![0; BadAddresses::size_of()];
        bincode::serialize_into(&mut data[..], slot_hashes).unwrap();
        mock_get_sysvar_syscall(&data);
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(5)]
    #[test_case(10)]
    #[test_case(64)]
    #[test_case(128)]
    #[test_case(192)]
    #[test_case(256)]
    #[test_case(384)]
    #[test_case(MAX_ENTRIES)]
    #[serial]
    fn test_pod_slot_hashes(num_entries: usize) {
        let mut bad_addresses = vec![];
        for _ in 0..num_entries {
            bad_addresses.push(Pubkey::new_unique());
        }

        let check_bad_addresses = BadAddresses::new(&bad_addresses);
        mock_bad_addresses(&check_bad_addresses);

        let pod_bad_addresses = PodBadAddresses::fetch().unwrap();

        // Assert the slice of `PodSlotHash` has the same length as
        // `SlotHashes`.
        let pod_slot_hashes_slice = pod_bad_addresses.as_slice().unwrap();
        assert_eq!(pod_slot_hashes_slice.len(), bad_addresses.len());

        // Assert `PodSlotHashes` and `SlotHashes` contain the same slot hashes
        // in the same order.
        for (i, bad_address) in bad_addresses.iter().enumerate() {
            assert_eq!(
                &pod_slot_hashes_slice[i].address, bad_address,
                "Mismatch at index {}",
                i
            );
        }

        // Check a few `None` values.
        // let not_a_addrs = Pubkey::new_unique();
        // assert_eq!(
        //     pod_slot_hashes.get(&not_a_slot).unwrap().as_ref(),
        //     check_slot_hashes.get(&not_a_slot),
        // );
        // assert_eq!(pod_slot_hashes.get(&not_a_slot).unwrap(), None);
        // assert_eq!(
        //     pod_slot_hashes.position(&not_a_slot).unwrap(),
        //     check_slot_hashes.position(&not_a_slot),
        // );
        // assert_eq!(pod_slot_hashes.position(&not_a_slot).unwrap(), None);

        // let not_a_slot = num_entries.saturating_add(2) as u64;
        // assert_eq!(
        //     pod_slot_hashes.get(&not_a_slot).unwrap().as_ref(),
        //     check_slot_hashes.get(&not_a_slot),
        // );
        // assert_eq!(pod_slot_hashes.get(&not_a_slot).unwrap(), None);
        // assert_eq!(
        //     pod_slot_hashes.position(&not_a_slot).unwrap(),
        //     check_slot_hashes.position(&not_a_slot),
        // );
        // assert_eq!(pod_slot_hashes.position(&not_a_slot).unwrap(), None);

        // Check a few `None` values.
        let no_address = BadAddresses::new(&[]);
        assert_eq!(no_address.get(), None)
    }

    // #[allow(deprecated)]
    // #[serial]
    // #[test]
    // fn test_slot_hashes_sysvar() {
    //     let mut slot_hashes = vec![];
    //     for i in 0..MAX_ENTRIES {
    //         slot_hashes.push((
    //             i as u64,
    //             hash(&[(i >> 24) as u8, (i >> 16) as u8, (i >> 8) as u8, i as u8]),
    //         ));
    //     }

    //     let check_slot_hashes = SlotHashes::new(&slot_hashes);
    //     mock_get_sysvar_syscall(&bincode::serialize(&check_slot_hashes).unwrap());

    //     // `get`:
    //     assert_eq!(
    //         SlotHashesSysvar::get(&0).unwrap().as_ref(),
    //         check_slot_hashes.get(&0),
    //     );
    //     assert_eq!(
    //         SlotHashesSysvar::get(&256).unwrap().as_ref(),
    //         check_slot_hashes.get(&256),
    //     );
    //     assert_eq!(
    //         SlotHashesSysvar::get(&511).unwrap().as_ref(),
    //         check_slot_hashes.get(&511),
    //     );
    //     // `None`.
    //     assert_eq!(
    //         SlotHashesSysvar::get(&600).unwrap().as_ref(),
    //         check_slot_hashes.get(&600),
    //     );

    //     // `position`:
    //     assert_eq!(
    //         SlotHashesSysvar::position(&0).unwrap(),
    //         check_slot_hashes.position(&0),
    //     );
    //     assert_eq!(
    //         SlotHashesSysvar::position(&256).unwrap(),
    //         check_slot_hashes.position(&256),
    //     );
    //     assert_eq!(
    //         SlotHashesSysvar::position(&511).unwrap(),
    //         check_slot_hashes.position(&511),
    //     );
    //     // `None`.
    //     assert_eq!(
    //         SlotHashesSysvar::position(&600).unwrap(),
    //         check_slot_hashes.position(&600),
    //     );
    // }
}
