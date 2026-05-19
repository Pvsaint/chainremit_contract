use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotFound = 1,
    Unauthorized = 2,
    AlreadyExists = 3,
    InsufficientCollateral = 4,
    InvalidAmount = 5,
    Expired = 6,
}
