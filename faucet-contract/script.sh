#!/bin/bash
set -e

./build.sh

near create-account faucet.icebear.testnet --masterAccount icebear.testnet --initialBalance 10
near deploy faucet.icebear.testnet --wasmFile out/faucet.wasm
# max share per account: 1000 ICB
near call faucet.icebear.testnet new '{"owner_id": "icebear.testnet", "ft_contract_id": "icb.icebear.testnet", "max_share": "1000000000000000000000000000"}' --accountId icebear.testnet
near view faucet.icebear.testnet get_faucet_info