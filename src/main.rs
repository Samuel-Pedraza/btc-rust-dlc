extern crate bitcoincore_rpc;

use bitcoin::util::uint::Uint256;
use bitcoincore_rpc::{Auth, Client, RpcApi};

const FUNDING_TRANSACTION_VERSION: i32 = 2;
const FUNDING_TRANSACTION_LOCKTIME: i32 = 0;

// negotiation process

// # A) Offer
// # B) Accept
// # C) Sign

// # A) "Alice" sends an offer_dlc message to "Bob"
// # B) "Bob" responds with an accept_dlc response
// # C) "Alice" then sends a sign_dlc message to Bob. Within that sign_dlc message we do the following:
// # C.1) Contract Execution Transcations are created
// # C.2) Refund Transaction is created
// # C.3) 
// i am currently here: https://github.com/discreetlogcontracts/dlcspecs/blob/master/Protocol.md

// create funding transaction

// create contract execution transaction


// create refund transaction


// create oracle to transact with

//
#[derive(Default)]
pub struct OfferDLC {
    offer_dlc_v0: Uint256,
    chain_hash: Hash,
    contract_info: String, //contract info specifies sender payouts for all events?
    oracle_info: String, //specifies oracles to be used (not sure how)
    point: String, //this actual a pubkey
    spk: String, //specifies script pubkey that CET and
    total_collateral_satoshi: u64,
    num_funding_inputs: u16,
    funding_inputs: total_collateral_satoshi * num_funding_inputs,
    change_spk: String, //pubkey that we're sending the change to
    feerate_per_vb: u64,
    contract_maturity_bound: u32,
    contract_timeout: u32,
}
// how do we send change out?
// how do we specify oracle? what is the oracles identity thingy?
// how does sender payouts work? what does that mean?
/* 
The receiving node MUST reject the contract if:

the chain_hash value is set to a hash of a chain that is unknown to the receiver.
the contract_info refers to events unknown to the receiver.
the oracle_info refers to an oracle unknown or inaccessible to the receiver.
it considers feerate_per_vb too small for timely processing or unreasonably large.
funding_pubkey is not a valid secp256k1 pubkey in compressed format.
funding_inputs do not contribute at least total_collateral_satoshis plus full fee payment.


*/

pub struct AcceptDLC {
    offer_type: Uint256,
    temporary_contract_id: u64,
    funding_pubkey: String, //actually a pubkey
    payout_spk: String, //actually pubkey i think
    funding_inputs: String, //same as offerdlc akshzuly
    change_spk: String, //pubkey same as offerdlc
    cet_adaptor_signatures: String, //same
    refund_signature: String,
}


fn main() {


    let rpc = Client::new("http://localhost:18332".to_string(),
                          Auth::UserPass("sam".to_string(),
                                         "password".to_string())).unwrap();

    let block = rpc.get_block_hash(0).unwrap();

    let genesis_block_information = rpc.get_block(&block);
    
    println!("{:?}", genesis_block_information);
}


fn generate_offer_message(){

    


}