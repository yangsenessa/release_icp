# release_icp
release_icp

# create smart-contract
dfx new release_contract && cd release_contract && dfx canister create release_contract

# latest version
550209a750be0575cc4afff5e23c04b03048c08e

https://raw.githubusercontent.com/dfinity/ic/550209a750be0575cc4afff5e23c04b03048c08e/rs/ledger_suite/icrc1/ledger/ledger.did

https://download.dfinity.systems/ic/550209a750be0575cc4afff5e23c04b03048c08e/canisters/ic-icrc1-ledger.wasm.gz

# Deploy Guide

## Prerequisites
- DFX CLI installed
- Install DFX CLI:
```bash
sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"
```
- Verify installation:
```bash
dfx --version
```
- Root access

## Token Configuration
- Token Symbol: "REV"
- Token Name: "Reverse"
- Initial Supply: 20,000,000,000,000,000
- Transfer Fee: 10

## Deployment Steps
1. Navigate to project directory
2. Run deploy script:
```bash
./deploy.sh
```
# Deploy.sh Process Documentation

This script handles the deployment of Reverse Token on the Internet Computer Protocol (ICP).

## Process Flow

1. **Initial Setup**
    - Changes directory to `release_contract`
    - Stops any running DFX instance
    - Starts DFX in background with clean state
    - Sets DFX_HOME to root directory

2. **Miner Identity Creation**
    - Creates/checks for REVERSE_MINER identity
    - Directory path: `/root/.config/dfx/identity/reverse_miner`
    - If identity doesn't exist, creates new one in plaintext mode
    - Switches to REVERSE_MINER identity
    - Retrieves REVERSE_MINER principal ID

3. **Governance Identity Creation**
    - Creates/checks for REVERSE_GOV identity
    - Directory path: `/root/.config/dfx/identity/reverse_gov`
    - If identity doesn't exist, creates new one in plaintext mode
    - Switches to REVERSE_GOV identity
    - Retrieves REVERSE_GOV principal ID

4. **Token Deployment**
    - Switches back to REVERSE_MINER identity
    - Deploys reverse_ledger_canister with following configurations:
      - Token Symbol: "REV"
      - Token Name: "Reverse"
      - Minting Account: REVERSE_MINER_PRINCIPAL
      - Transfer Fee: 10
      - Initial Balance: 20,000,000,000,000,000 tokens allocated to REVERSE_GOV_PRINCIPAL
      - Archive Options:
         - Blocks to archive: 1000
         - Trigger threshold: 2000
         - Controller: REVERSE_GOV_PRINCIPAL
      - ICRC2 feature flag enabled

5. **Index Canister Setup**
    - Exports REVERSE_TOKEN_CANISTER_ID
    - Deploys reverse_index_canister linked to the ledger canister

6. **Verification**
    - Checks balance of GOV_ACCOUNT using icrc1_balance_of call
    - Confirms setup completion

## Important Notes
- Uses `set -e` to exit on any error
- Stores identities in plaintext mode for development purposes
- Implements ICRC1 and ICRC2 token standards
- Creates two main roles: Miner and Governance


## Canister IDs
- Reverse Token: `mxzaz-hqaaa-aaaar-qaada-cai`
- Index Canister: `76f3y-fyaaa-aaaah-qpxxa-cai`

## Identity Management
The deploy script automatically creates:
- Miner identity
- Governance identity

## Verification
Check deployment by querying governance balance:
```bash
dfx canister call reverse_ledger_canister icrc1_balance_of
```
