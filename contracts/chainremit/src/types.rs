use soroban_sdk::{contracttype, Address, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransferStatus {
    Pending,
    Completed,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferEntry {
    pub sender: Address,
    pub recipient: Address,
    pub amount: i128,
    pub fee: i128,
    pub status: TransferStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoanStatus {
    Active,
    Repaid,
    Defaulted,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoanEntry {
    pub borrower: Address,
    pub principal: i128,
    pub collateral: i128,
    pub interest_rate: u32,
    pub due_date: u64,
    pub status: LoanStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavingsGroupEntry {
    pub members: Vec<Address>,
    pub contribution_amount: i128,
    pub cycle_duration: u64,
    pub current_cycle: u32,
    pub total_pooled: i128,
}
