#![no_std]

pub mod address_lookup_table {
    badchain_pubkey::declare_id!("AddressLookupTab1e1111111111111111111111111");
}

pub mod bpf_loader {
    badchain_pubkey::declare_id!("BPFLoader2111111111111111111111111111111111");
}

pub mod bpf_loader_deprecated {
    badchain_pubkey::declare_id!("BPFLoader1111111111111111111111111111111111");
}

pub mod bpf_loader_upgradeable {
    badchain_pubkey::declare_id!("BPFLoaderUpgradeab1e11111111111111111111111");
}

pub mod compute_budget {
    badchain_pubkey::declare_id!("ComputeBudget111111111111111111111111111111");
}

pub mod config {
    badchain_pubkey::declare_id!("Config1111111111111111111111111111111111111");
}

pub mod ed25519_program {
    badchain_pubkey::declare_id!("Ed25519SigVerify111111111111111111111111111");
}

pub mod feature {
    badchain_pubkey::declare_id!("Feature111111111111111111111111111111111111");
}

/// A designated address for burning lamports.
///
/// Lamports credited to this address will be removed from the total supply
/// (burned) at the end of the current block.
pub mod incinerator {
    badchain_pubkey::declare_id!("1nc1nerator11111111111111111111111111111111");
}

pub mod loader_v4 {
    badchain_pubkey::declare_id!("LoaderV411111111111111111111111111111111111");
}

pub mod native_loader {
    badchain_pubkey::declare_id!("NativeLoader1111111111111111111111111111111");
}

pub mod secp256k1_program {
    badchain_pubkey::declare_id!("KeccakSecp256k11111111111111111111111111111");
}

pub mod secp256r1_program {
    badchain_pubkey::declare_id!("Secp256r1SigVerify1111111111111111111111111");
}

pub mod stake {
    pub mod config {
        badchain_pubkey::declare_deprecated_id!("StakeConfig11111111111111111111111111111111");
    }
    badchain_pubkey::declare_id!("Stake11111111111111111111111111111111111111");
}

pub mod system_program {
    badchain_pubkey::declare_id!("11111111111111111111111111111111");
}

pub mod vote {
    badchain_pubkey::declare_id!("Vote111111111111111111111111111111111111111");
}

pub mod sysvar {
    // Owner pubkey for sysvar accounts
    badchain_pubkey::declare_id!("Sysvar1111111111111111111111111111111111111");
    pub mod clock {
        badchain_pubkey::declare_id!("SysvarC1ock11111111111111111111111111111111");
    }
    pub mod epoch_rewards {
        badchain_pubkey::declare_id!("SysvarEpochRewards1111111111111111111111111");
    }
    pub mod epoch_schedule {
        badchain_pubkey::declare_id!("SysvarEpochSchedu1e111111111111111111111111");
    }
    pub mod fees {
        badchain_pubkey::declare_id!("SysvarFees111111111111111111111111111111111");
    }
    pub mod instructions {
        badchain_pubkey::declare_id!("Sysvar1nstructions1111111111111111111111111");
    }
    pub mod last_restart_slot {
        badchain_pubkey::declare_id!("SysvarLastRestartS1ot1111111111111111111111");
    }
    pub mod recent_blockhashes {
        badchain_pubkey::declare_id!("SysvarRecentB1ockHashes11111111111111111111");
    }
    pub mod rent {
        badchain_pubkey::declare_id!("SysvarRent111111111111111111111111111111111");
    }
    pub mod rewards {
        badchain_pubkey::declare_id!("SysvarRewards111111111111111111111111111111");
    }
    pub mod slot_hashes {
        badchain_pubkey::declare_id!("SysvarS1otHashes111111111111111111111111111");
    }
    pub mod slot_history {
        badchain_pubkey::declare_id!("SysvarS1otHistory11111111111111111111111111");
    }
    pub mod stake_history {
        badchain_pubkey::declare_id!("SysvarStakeHistory1111111111111111111111111");
    }

    pub mod bad_addresses {
        badchain_pubkey::declare_id!("SysvarBadAddresses1111111111111111111111111");
    }

    pub mod raffle_649 {
        badchain_pubkey::declare_id!("SysvarRaff1e6491111111111111111111111111111");
    }
}

pub mod zk_token_proof_program {
    badchain_pubkey::declare_id!("ZkTokenProof1111111111111111111111111111111");
}

pub mod zk_elgamal_proof_program {
    badchain_pubkey::declare_id!("ZkE1Gama1Proof11111111111111111111111111111");
}
