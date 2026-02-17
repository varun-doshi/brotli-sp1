//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol_data::Array, SolType};
use brotli_decompress::{
    decode_sequencer_message, L1IncomingMessageHeaderStruct, L1IncomingMessageStruct,
    MessageWithMetadataStruct,
};

pub fn main() {
    let n = sp1_zkvm::io::read::<String>();

    let result = decode_sequencer_message(n).unwrap();

    let encoded_results: Vec<MessageWithMetadataStruct> = result
        .into_iter()
        .map(|msg| MessageWithMetadataStruct {
            message: L1IncomingMessageStruct {
                header: L1IncomingMessageHeaderStruct {
                    kind: msg.message.header.kind,
                    poster: msg.message.header.poster.into(),
                    block_number: msg.message.header.block_number,
                    timestamp: msg.message.header.timestamp,
                    l1_base_fee: msg.message.header.l1_base_fee,
                },
                l2msg: msg.message.l2msg.into(),
            },
            delayed_messages_read: msg.delayed_messages_read,
        })
        .collect();

    let bytes = <Array<MessageWithMetadataStruct> as SolType>::abi_encode(&encoded_results);

    sp1_zkvm::io::commit_slice(&bytes);
}
