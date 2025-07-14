use solana_sdk::declare_id;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::sysvar::{Sysvar, SysvarId};
use std::mem::MaybeUninit;

declare_id!("SysvarRaff1e6491111111111111111111111111111");

#[repr(C)] // Ensures predictable memory layout for FFI
pub struct Raffle;

impl SysvarId for Raffle {
    fn id() -> Pubkey {
        id()
    }

    fn check_id(pubkey: &Pubkey) -> bool {
        pubkey == &id()
    }
}

// Syscall hook provided by runtime (must be linked in Solana BPF env)
extern "C" {
    fn sol_get_raffle_sysvar(ptr: *mut Raffle);
}

#[cfg(feature = "bincode")]
impl Sysvar for Raffle {
    fn get() -> Self {
        unsafe {
            let mut obj = MaybeUninit::<Self>::uninit();
            sol_get_raffle_sysvar(obj.as_mut_ptr());
            obj.assume_init()
        }
    }
}
