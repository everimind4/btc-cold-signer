use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub network: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub txid: String,
    pub vout: u32,
    pub value_sat: u64, 
    pub private_key_wif: String,
    pub script_pubkey: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub address: String,
    pub value_sat: u64,
}
