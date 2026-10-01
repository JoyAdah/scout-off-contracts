#![allow(deprecated, dead_code)]
use scoutchain_shared_types::ProgressLevel;
use soroban_sdk::{Address, BytesN, Env, Symbol};

pub const ADMIN_TRANSFERRED: &str = "admin_transferred";
pub const ADMIN_TRANSFER_PROPOSED: &str = "admin_transfer_proposed";
pub const PROGRESS_UPDATED: &str = "progress_updated";
pub const PLAYER_LEVEL_RESET: &str = "player_level_reset";
pub const CONTRACT_UPGRADED: &str = "contract_upgraded";
pub const WIRING_UPDATED: &str = "wiring_updated";
pub const MIGRATION_WINDOW_OPENED: &str = "migration_window_opened";
pub const MIGRATION_WINDOW_CLOSED: &str = "migration_window_closed";

/// topics: (event_name, old_admin)  data: new_admin
pub fn admin_transferred(env: &Env, old_admin: &Address, new_admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "admin_transferred"), old_admin.clone()),
        new_admin.clone(),
    );
}

/// topics: (event_name, old_admin)  data: new_admin
pub fn admin_transfer_proposed(env: &Env, old_admin: &Address, new_admin: &Address) {
    env.events().publish(
        (Symbol::new(env, ADMIN_TRANSFER_PROPOSED), old_admin.clone()),
        new_admin.clone(),
    );
}

/// topics: (event_name, updated_by)  data: (player_id, old_level, new_level)
pub fn progress_updated(
    env: &Env,
    player_id: u64,
    old_level: &ProgressLevel,
    new_level: &ProgressLevel,
    updated_by: &Address,
) {
    env.events().publish(
        (Symbol::new(env, PROGRESS_UPDATED), updated_by.clone()),
        (player_id, old_level.clone(), new_level.clone()),
    );
}

pub fn player_level_reset(env: &Env, admin: &Address, player_id: u64, old_level: &ProgressLevel) {
    env.events().publish(
        (Symbol::new(env, PLAYER_LEVEL_RESET), admin.clone()),
        (player_id, old_level.clone()),
    );
}

pub fn wiring_updated(env: &Env, admin: &Address, target: &Symbol, new_address: &Address) {
    env.events().publish(
        (Symbol::new(env, WIRING_UPDATED), admin.clone()),
        (target.clone(), new_address.clone()),
    );
}

pub fn migration_window_opened(env: &Env, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, MIGRATION_WINDOW_OPENED), admin.clone()),
        (),
    );
}

pub fn migration_window_closed(env: &Env, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, MIGRATION_WINDOW_CLOSED), admin.clone()),
        (),
    );
}

/// Emitted before `update_current_contract_wasm` — attributed to the old code version.
/// topics: (event_name, admin)  data: new_wasm_hash
pub fn contract_upgraded(env: &Env, admin: &Address, new_wasm_hash: &BytesN<32>) {
    env.events().publish(
        (Symbol::new(env, CONTRACT_UPGRADED), admin.clone()),
        new_wasm_hash.clone(),
    );
}
