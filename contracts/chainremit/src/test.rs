#![cfg(test)]

use super::*;
use crate::storage::{
    read_loan, read_savings_group, read_transfer, write_loan, write_savings_group, write_transfer,
};
use crate::types::{LoanEntry, LoanStatus, SavingsGroupEntry, TransferEntry, TransferStatus};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{symbol_short, vec, Address, Env, Vec};

#[test]
fn test_hello() {
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}

#[test]
fn test_transfer_storage_round_trip() {
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let id: soroban_sdk::BytesN<32> = env
        .crypto()
        .sha256(&soroban_sdk::Bytes::from_slice(&env, &[1u8; 64]))
        .into();

    env.as_contract(&contract_id, || {
        assert!(read_transfer(&env, &id).is_none());

        let entry = TransferEntry {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount: 1000,
            fee: 5,
            status: TransferStatus::Pending,
        };
        write_transfer(&env, &id, &entry);

        let stored = read_transfer(&env, &id).expect("Transfer should exist after write");
        assert_eq!(stored.sender, sender);
        assert_eq!(stored.recipient, recipient);
        assert_eq!(stored.amount, 1000);
        assert_eq!(stored.fee, 5);
        assert_eq!(stored.status, TransferStatus::Pending);
    });
}

#[test]
fn test_loan_storage_round_trip() {
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());

    let borrower = Address::generate(&env);

    env.as_contract(&contract_id, || {
        assert!(read_loan(&env, &borrower).is_none());

        let entry = LoanEntry {
            borrower: borrower.clone(),
            principal: 5000,
            collateral: 6000,
            interest_rate: 5,
            due_date: 100000,
            status: LoanStatus::Active,
        };
        write_loan(&env, &borrower, &entry);

        let stored = read_loan(&env, &borrower).expect("Loan should exist after write");
        assert_eq!(stored.borrower, borrower);
        assert_eq!(stored.principal, 5000);
        assert_eq!(stored.collateral, 6000);
        assert_eq!(stored.interest_rate, 5);
        assert_eq!(stored.due_date, 100000);
        assert_eq!(stored.status, LoanStatus::Active);
    });
}

#[test]
fn test_savings_group_storage_round_trip() {
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());

    let member1 = Address::generate(&env);
    let member2 = Address::generate(&env);
    let mut members = Vec::new(&env);
    members.push_back(member1.clone());
    members.push_back(member2.clone());

    let group_id = symbol_short!("CHAMA");

    env.as_contract(&contract_id, || {
        assert!(read_savings_group(&env, &group_id).is_none());

        let entry = SavingsGroupEntry {
            members: members.clone(),
            contribution_amount: 500,
            cycle_duration: 86400,
            current_cycle: 1,
            total_pooled: 1000,
        };
        write_savings_group(&env, &group_id, &entry);

        let stored =
            read_savings_group(&env, &group_id).expect("Savings group should exist after write");
        assert_eq!(stored.members.len(), 2);
        assert_eq!(stored.contribution_amount, 500);
        assert_eq!(stored.cycle_duration, 86400);
        assert_eq!(stored.current_cycle, 1);
        assert_eq!(stored.total_pooled, 1000);
    });
}
