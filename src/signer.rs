use crate::param::{Params, Input};
use bitcoin::{Transaction, PrivateKey, Amount, ScriptBuf, EcdsaSighashType, ecdsa};
use bitcoin::sighash::SighashCache;
use bitcoin::secp256k1::{Secp256k1, Message};
use bitcoin::hashes::Hash;

pub fn sign_tx(tx: Transaction, params: &Params) -> String {
    let secp = Secp256k1::new();
    let mut sighash_cache = SighashCache::new(tx);

    for (index, input) in params.inputs.iter().enumerate() {
        sign_input(&secp, &mut sighash_cache, input, index);
    }

    let signed_tx = sighash_cache.into_transaction();
    bitcoin::consensus::encode::serialize_hex(&signed_tx)
}

fn sign_input(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    sighash_cache: &mut SighashCache<Transaction>,
    input: &Input,
    index: usize,
) {
    let private_key = PrivateKey::from_wif(
        &input.private_key_wif
    ).expect("Invalid WIF private key");
    
    let script_pubkey = ScriptBuf::from_hex(
        &input.script_pubkey
    ).expect("Invalid script_pubkey hex");

    let sighash = sighash_cache.p2wpkh_signature_hash(
        index,
        &script_pubkey,
        Amount::from_sat(input.value_sat),
        EcdsaSighashType::All,
    ).expect("Failed to compute sighash");

    let message = Message::from_digest(*sighash.as_byte_array());
    let signature = secp.sign_ecdsa(&message, &private_key.inner);

    // Wrap secp256k1::ecdsa::Signature into bitcoin::ecdsa::Signature,
    // which additionally carries the sighash type required by P2WPKH witness
    let bitcoin_sig = ecdsa::Signature {
        signature,
        sighash_type: EcdsaSighashType::All,
    };

    let public_key = private_key.public_key(&secp);
    let witness = sighash_cache
        .witness_mut(index)
        .expect(&format!("Failed to access witness for input {}", index));
    *witness = bitcoin::Witness::p2wpkh(
        &bitcoin_sig,
        &public_key.inner,
    );
}
