use crate::param::Params;
use bitcoin::{Transaction, PrivateKey, Amount, ScriptBuf, EcdsaSighashType, ecdsa};
use bitcoin::sighash::SighashCache;
use bitcoin::secp256k1::{Secp256k1, Message};
use bitcoin::hashes::Hash;

pub fn sign_tx(tx: Transaction, params: &Params) -> String {
    let private_key = PrivateKey::from_wif(
        &params.inputs[0].private_key_wif
    ).expect("Invalid WIF private key");
    
    let script_pubkey = ScriptBuf::from_hex(
        &params.inputs[0].script_pubkey
    ).expect("Invalid script_pubkey hex");

    let mut sighash_cache = SighashCache::new(tx);

    let sighash = sighash_cache.p2wpkh_signature_hash(
        0,
        &script_pubkey,
        Amount::from_sat(params.inputs[0].value_sat),
        EcdsaSighashType::All,
    ).expect("Failed to compute sighash");

    let secp = Secp256k1::new();
    let message = Message::from_digest(*sighash.as_byte_array());
    let signature = secp.sign_ecdsa(&message, &private_key.inner);

    let bitcoin_sig = ecdsa::Signature {
        signature,
        sighash_type: EcdsaSighashType::All,
    };

    let public_key = private_key.public_key(&secp);
    let witness = sighash_cache
        .witness_mut(0)
        .expect("Failed to access witness for input 0");
    *witness = bitcoin::Witness::p2wpkh(
        &bitcoin_sig,
        &public_key.inner,
    );

    let signed_tx = sighash_cache.into_transaction();
    bitcoin::consensus::encode::serialize_hex(&signed_tx)
}
