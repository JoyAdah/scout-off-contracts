#![no_std]

use soroban_sdk::{contracttype, Address, Env, IntoVal, String};

/// Four-tier progress level for a player profile
#[contracttype]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProgressLevel {
    /// Level 0 - profile created, no verification yet
    Unverified,
    /// Level 1 - identity confirmed by academy or KYC
    VerifiedIdentity,
    /// Level 2 - performance milestones verified by approved third party
    PerformanceMilestones,
    /// Level 3 - scout feedback or trial offer logged
    EliteTier,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ContractHealth {
    /// Whether the contract has completed its one-time initialization.
    pub initialized: bool,
    /// Whether state-changing operations are currently paused.
    pub paused: bool,
    /// Whether the `scout_access.pay_to_contact` function is paused independently
    /// of the whole-contract pause (function-scoped circuit breaker).
    /// Always `false` for contracts that do not implement a `pay_to_contact`
    /// function (`registration`, `verification`, `progress`).
    pub pay_to_contact_paused: bool,
    /// Whether the one-time migration window is currently open.
    /// When `true`, admin can seed historical data via `admin_seed_*` functions.
    /// Once closed via `close_migration_window`, this window can never be reopened
    /// (the `MigrationWindowSealed` flag is set permanently).
    pub migration_window_open: bool,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct WiringLink {
    /// The peer contract address. Empty if not yet configured.
    pub address: Address,
    /// Monotonic re-wiring epoch, incremented on every successful re-wiring.
    /// Used to detect stale wiring references after upgrades or admin rotations.
    pub epoch: u32,
}

impl WiringLink {
    pub fn new(address: Address, epoch: u32) -> Self {
        Self { address, epoch }
    }

    pub fn empty() -> Self {
        Self {
            address: Address::from_str(&Env::default(), ""),
            epoch: 0,
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.address.to_string().is_empty()
    }
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ContractHealthOld {
    /// Whether the contract has completed its one-time initialization.
    pub initialized: bool,
    /// Whether state-changing operations are currently paused.
    pub paused: bool,
    /// Whether the `scout_access.pay_to_contact` function is paused independently
    /// of the whole-contract pause (function-scoped circuit breaker).
    /// Always `false` for contracts that do not implement a `pay_to_contact`
    /// function (`registration`, `verification`, `progress`).
    pub pay_to_contact_paused: bool,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum AdminError {
    NotInitialized,
    AlreadyInitialized,
    Unauthorized,
}

pub trait AdminError {
    fn not_initialized() -> Self;
    fn already_initialized() -> Self {
        panic!("already_initialized not implemented")
    }
    fn unauthorized() -> Self {
        panic!("unauthorized not implemented")
    }
}