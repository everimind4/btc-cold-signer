# btc-cold-signer 🔐

Offline Bitcoin transaction signer (CLI). Signs a P2WPKH transaction using a private key and UTXO info provided via params.json, with no network access required during signing.

## Usage

```bash
# 1. Prepare params.json (see params.example.json)

cp params.example.json params.json
# Fill in your UTXO info and private key

# 2. Sign (offline - no network required)
cargo run -- params.json

# 3. Broadcast the output hex at:
#    https://mempool.space/testnet/tx/push
```
## Test Environment

- Network: testnet
- Test wallet address: tb1qww7rp2vvv9m7yj4p5d7hrh52cwktg33wgge6us
- Funded via: coinfaucet.eu
- Funding tx: d227406f3791bb747da886f2d73adf18730e82b209c3a15981bac6c58880983c (vout 1) — confirmed

## Design: Air-gapped Signing

This tool follows the cold wallet model: network-facing tasks (UTXO lookup, fee estimation) are performed externally by the operator, and the results are passed in via params.json.
The signer itself contains no network code, ensuring the private key never touches an internet-connected process. Fee is calculated as the implicit difference between total input and total output value.

## Broadcast Result

A sample execution result is available at:
https://mempool.space/testnet/tx/52bb87ad340f32f5d502d0655f2a457f96e83797ddb7b5a78e0f84e2ea5dd0a0
