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
                txid: Txid::from_str(&input.txid).unwrap(),
                vout: input.vout,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }
    }).collect();

    let tx_outputs: Vec<TxOut> = params.outputs.iter().map(|output| {
        let address = Address::from_str(&output.address).unwrap()
                      .require_network(network).unwrap();
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
