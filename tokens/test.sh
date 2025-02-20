set -e
cd release_contract
# REVERSE_MAIN_SITE_PRINCIPAL: fs7xc-hl64c-g3bt5-r2k67-txuht-sn5bp-lzfas-pdqrj-tmm64-mbahx-zae
# REVERSE_GOV_PRINCIPAL: 4zbyc-zoe6z-gbmu3-5ewrb-nnfeo-2a5jj-qzz2s-qkyqf-3j3z4-khjqz-jqe
# REVERSE_MINER_PRINCIPAL: ozvt2-nr7fe-co6wl-76lzl-t4cma-m7p4t-6j43w-vh67m-6vr3b-d3f7d-yae


# Burn tokens
dfx canister call reverse_ledger_canister icrc1_transfer "(record {
    to=record {
        owner=principal \"ozvt2-nr7fe-co6wl-76lzl-t4cma-m7p4t-6j43w-vh67m-6vr3b-d3f7d-yae\"
    }; 
    amount=10000000000
})"

# ICRC2 payment tests
dfx canister call reverse_ledger_canister icrc2_approve "(record {
    amount=20000000000;
    spender=record {
        owner=principal \"fs7xc-hl64c-g3bt5-r2k67-txuht-sn5bp-lzfas-pdqrj-tmm64-mbahx-zae\"
    }
})"

dfx canister call reverse_ledger_canister icrc2_allowance "(record {
    account=record {
        owner=principal \"4zbyc-zoe6z-gbmu3-5ewrb-nnfeo-2a5jj-qzz2s-qkyqf-3j3z4-khjqz-jqe\"
    };
    spender=record {
        owner=principal \"fs7xc-hl64c-g3bt5-r2k67-txuht-sn5bp-lzfas-pdqrj-tmm64-mbahx-zae\"
    }
})"


