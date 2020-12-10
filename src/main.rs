extern crate bitcoincore_rpc;

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


fn main() {

    let rpc = Client::new("http://localhost:18332".to_string(),
                          Auth::UserPass("sam".to_string(),
                                         "password".to_string())).unwrap();

    let block = rpc.get_block_hash(0).unwrap();

    let genesis_block_information = rpc.get_block(&block);
    
    println!("{:?}", genesis_block_information);
}