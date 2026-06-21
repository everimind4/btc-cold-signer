## Test Environment

- Network: testnet
- Test wallet address: tb1qww7rp2vvv9m7yj4p5d7hrh52cwktg33wgge6us
- Funded via: coinfaucet.eu
- Funding tx: d227406f3791bb747da886f2d73adf18730e82b209c3a15981bac6c58880983c (vout 1) — confirmed

## Design: Air-gapped Signing

This tool follows the cold wallet model: network-facing tasks
(UTXO lookup, fee estimation) are performed externally by the
operator, and the results are passed in
via params.json. The signer itself contains no network code,
ensuring the private key never touches an internet-connected
process. Fee is calculated as the implicit
difference between total input and total output value.
