#!/bin/bash
set -e

./build.sh

near create-account icb.icebear.testnet --masterAccount icebear.testnet --initialBalance 10
near deploy icb.icebear.testnet --wasmFile out/ft.wasm
# init ICB token contract with 1M total supply (24 decimals)
near call icb.icebear.testnet new_default_meta '{"owner_id": "icebear.testnet", "total_supply": "1000000000000000000000000000000"}' --accountId icebear.testnet

near view icb.icebear.testnet ft_metadata

# transfer to faucet contract (make sure you deployed faucet contract before calling this command)
near call icb.icebear.testnet ft_transfer_call '{"receiver_id": "faucet.icebear.testnet", "amount": "1000000000000000000000000000000", "msg": ""}' --accountId icebear.testnet --depositYocto 1 --gas 50000000000000