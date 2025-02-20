use candid::{Nat, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::{query, update};

use crate::icrc_payment_types::{
    Account, Allowance, ApproveArgs, ApproveResult, TransferArgs, TransferFromArgs,
    TransferFromResult, TransferResult,
};

const LEDGER_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";

#[update]
async fn icrc1_transfer(args: TransferArgs) -> TransferResult {
    let ledger = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    let result: CallResult<(TransferResult,)> = 
        ic_cdk::call(ledger, "icrc1_transfer", (args,)).await;
    
    match result {
        Ok((transfer_result,)) => transfer_result,
        Err(e) => TransferResult::Err(e.into()),
    }
}

#[update]
async fn icrc2_approve(args: ApproveArgs) -> ApproveResult {
    let ledger = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    let result: CallResult<(ApproveResult,)> = 
        ic_cdk::call(ledger, "icrc2_approve", (args,)).await;
    
    match result {
        Ok((approve_result,)) => approve_result,
        Err(e) => ApproveResult::Err(e.into()),
    }
}

#[update]
async fn icrc2_transfer_from(args: TransferFromArgs) -> TransferFromResult {
    let ledger = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    let result: CallResult<(TransferFromResult,)> = 
        ic_cdk::call(ledger, "icrc2_transfer_from", (args,)).await;
    
    match result {
        Ok((transfer_result,)) => transfer_result,
        Err(e) => TransferFromResult::Err(e.into()),
    }
}

#[query]
async fn icrc1_balance_of(account: Account) -> Nat {
    let ledger = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    let result: CallResult<(Nat,)> = 
        ic_cdk::call(ledger, "icrc1_balance_of", (account,)).await;
    
    result.unwrap_or((Nat::from(0),)).0
}

#[query]
async fn icrc2_allowance(owner: Account, spender: Account) -> Allowance {
    let ledger = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    let args = (owner, spender);
    let result: CallResult<(Allowance,)> = 
        ic_cdk::call(ledger, "icrc2_allowance", (args,)).await;
    
    result.unwrap_or((Allowance { 
        allowance: Nat::from(0), 
        expires_at: None 
    },)).0
}

// Helper function to convert string amounts to Nat
pub fn string_to_nat(amount: String) -> Nat {
    Nat::from_str(&amount).unwrap_or(Nat::from(0))
}

// Helper function to convert principal strings to Principal
pub fn string_to_principal(principal_str: String) -> Result<Principal, String> {
    Principal::from_text(principal_str)
        .map_err(|e| format!("Invalid principal: {}", e))
}

// Helper function to convert hex string to subaccount bytes
pub fn hex_to_subaccount(hex: String) -> Option<Vec<u8>> {
    if hex.is_empty() {
        return None;
    }
    
    let hex = hex.trim_start_matches("0x");
    let bytes = hex::decode(hex).ok()?;
    Some(bytes)
}
