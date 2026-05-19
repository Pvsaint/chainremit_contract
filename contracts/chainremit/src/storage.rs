use soroban_sdk::{contracttype, Address, BytesN, Env, Symbol};

use crate::types::{LoanEntry, SavingsGroupEntry, TransferEntry};

// ---------------------------------------------------------------------------
// Ledger key definitions
// ---------------------------------------------------------------------------

/// All storage keys used by the chainremit contract.
#[contracttype]
#[derive(Clone, Debug)]
pub enum LedgerKey {
    /// Primary transfer record keyed by the unique transfer ID (SHA-256 hash).
    Transfer(BytesN<32>),
    /// Loan record keyed by the borrower's address (assuming one active loan).
    Loan(Address),
    /// Savings group record keyed by the unique group identifier/name.
    SavingsGroup(Symbol),
}

// ---------------------------------------------------------------------------
// Transfer storage helpers
// ---------------------------------------------------------------------------

/// Read a transfer entry from persistent storage. Returns `None` if not found.
pub fn read_transfer(env: &Env, id: &BytesN<32>) -> Option<TransferEntry> {
    env.storage()
        .persistent()
        .get(&LedgerKey::Transfer(id.clone()))
}

/// Write (create or overwrite) a transfer entry in persistent storage.
pub fn write_transfer(env: &Env, id: &BytesN<32>, entry: &TransferEntry) {
    env.storage()
        .persistent()
        .set(&LedgerKey::Transfer(id.clone()), entry);
}

// ---------------------------------------------------------------------------
// Loan storage helpers
// ---------------------------------------------------------------------------

/// Read a loan entry from persistent storage. Returns `None` if not found.
pub fn read_loan(env: &Env, borrower: &Address) -> Option<LoanEntry> {
    env.storage()
        .persistent()
        .get(&LedgerKey::Loan(borrower.clone()))
}

/// Write (create or overwrite) a loan entry in persistent storage.
pub fn write_loan(env: &Env, borrower: &Address, entry: &LoanEntry) {
    env.storage()
        .persistent()
        .set(&LedgerKey::Loan(borrower.clone()), entry);
}

// ---------------------------------------------------------------------------
// Savings group storage helpers
// ---------------------------------------------------------------------------

/// Read a savings group entry from persistent storage. Returns `None` if not found.
pub fn read_savings_group(env: &Env, id: &Symbol) -> Option<SavingsGroupEntry> {
    env.storage()
        .persistent()
        .get(&LedgerKey::SavingsGroup(id.clone()))
}

/// Write (create or overwrite) a savings group entry in persistent storage.
pub fn write_savings_group(env: &Env, id: &Symbol, entry: &SavingsGroupEntry) {
    env.storage()
        .persistent()
        .set(&LedgerKey::SavingsGroup(id.clone()), entry);
}
