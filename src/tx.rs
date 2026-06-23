use crate::param::Params;
use bitcoin::{Transaction, TxIn, Txid, TxOut, OutPoint, Amount, Address, Network, Sequence, Witness, ScriptBuf, transaction::Version, absolute::LockTime};
use std::str::FromStr;

pub fn build_tx(params: &Params) -> Transaction { 
    let network = match params.network.as_str() {
        "mainnet" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        _ => panic!("Not supported network: {}", params.network),
    };

    let tx_inputs: Vec<TxIn> = params.inputs.iter().map(|input| {
        TxIn {
            previous_output: OutPoint {
                txid: Txid::from_str(&input.txid)
                            .expect("Invalid txid hex"),
                vout: input.vout,
            },
            script_sig: ScriptBuf::new(),   // empty: SegWit uses witness instead
            sequence: Sequence::MAX,
            witness: Witness::new(),        // to be filled in by signer
        }
    }).collect();

    let tx_outputs: Vec<TxOut> = params.outputs.iter().map(|output| {
        let address = Address::from_str(&output.address).expect("Invalid address format")
                               .require_network(network).expect("Address does not match the specified network");
        TxOut {
            value: Amount::from_sat(output.value_sat),
            script_pubkey: address.script_pubkey(),
        }
    }).collect();

    Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: tx_inputs,
        output: tx_outputs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::param::{Params, Input, Output};

    fn make_params() -> Params {
        Params { // Same as sample
            network: "testnet".to_string(),
            inputs: vec![Input {
                txid: "d227406f3791bb747da886f2d73adf18730e82b209c3a15981bac6c58880983c".to_string(),
                vout: 1,
                value_sat: 304896,
                private_key_wif: "dummy_private_key".to_string(), // not used in build_tx
                script_pubkey: "001473bc30a98c6177e24aa1a37d71de8ac3acb4462e".to_string(),
            }],
            outputs: vec![
                Output {
                    address: "tb1qerzrlxcfu24davlur5sqmgzzgsal6wusda40er".to_string(),
                    value_sat: 280000,
                },
                Output {
                    address: "tb1qww7rp2vvv9m7yj4p5d7hrh52cwktg33wgge6us".to_string(),
                    value_sat: 20000,
                },
            ],
        }
    }

    #[test]
    fn test_input_count_matches_params() {
        let params = make_params();
        let tx = build_tx(&params);
        assert_eq!(tx.input.len(), params.inputs.len());
    }

    #[test]
    fn test_output_count_matches_params() {
        let params = make_params();
        let tx = build_tx(&params);
        assert_eq!(tx.output.len(), params.outputs.len());
    }

    #[test]
    fn test_output_amounts_match_params() {
        let params = make_params();
        let tx = build_tx(&params);
        for (i, output) in params.outputs.iter().enumerate() {
            assert_eq!(
                tx.output[i].value.to_sat(),
                output.value_sat,
                "output[{}] amount mismatch", i
            );
        }
    }

    #[test]
    fn test_outputs_do_not_exceed_inputs() {
        let params = make_params();
        let total_in: u64 = params.inputs.iter().map(|i| i.value_sat).sum();
        let total_out: u64 = params.outputs.iter().map(|o| o.value_sat).sum();
        assert!(total_out < total_in, "total outputs must be less than total inputs");
    }
}
