use alloy_sol_types::sol;
use anyhow::{bail, Context, Ok, Result};
use brotli_decompressor::DecompressorWriter;
use rlp::{PayloadInfo, Rlp};
use std::{
    fmt::{Debug, Display},
    io::Write,
    u128,
};

// sol! {
//     /// The public values encoded as a struct that can be easily deserialized inside Solidity.
//     struct PublicValuesStruct {
//         uint32 n;
//         uint32 a;
//         uint32 b;
//     }
// }

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
// pub fn fibonacci(n: u32) -> (u32, u32) {
//     let mut a = 0u32;
//     let mut b = 1u32;
//     for _ in 0..n {
//         let c = a.wrapping_add(b);
//         a = b;
//         b = c;
//     }
//     (a, b)
// }

sol! {
    /// The header of an L1 incoming message.
    struct L1IncomingMessageHeaderStruct {
        uint8 kind;
        address poster;
        uint64 block_number;
        uint64 timestamp;
        uint128 l1_base_fee;
    }

    /// An L1 incoming message, which includes a header and the L2 message bytes.
    struct L1IncomingMessageStruct {
        L1IncomingMessageHeaderStruct header;
        bytes l2msg;
    }

    /// The public values struct for the sequencer message decoding circuit.
    struct MessageWithMetadataStruct {
        L1IncomingMessageStruct message;
        uint64 delayed_messages_read;
    }

    // type MessageWithMetadataArray is <Array<MessageWithMetadataStruct>>;
}

#[derive(Debug, Clone)]
pub struct L1IncomingMessageHeader {
    pub kind: u8,
    pub poster: [u8; 20],
    pub block_number: u64,
    pub timestamp: u64,
    pub l1_base_fee: u128,
}

#[derive(Clone)]
pub struct L1IncomingMessage {
    pub header: L1IncomingMessageHeader,
    pub l2msg: Vec<u8>,
}

impl Debug for L1IncomingMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "L1IncomingMessage {{ kind: {}, poster: 0x{}, block_number: {}, timestamp: {}, l1_base_fee: {}, l2msg: 0x{} }}",
            self.header.kind,
            hex::encode(self.header.poster),
            self.header.block_number,
            self.header.timestamp,
            self.header.l1_base_fee,
            hex::encode(&self.l2msg),
        )
    }
}

#[derive(Debug, Clone)]
pub struct MessageWithMetadata {
    pub message: L1IncomingMessage,
    pub delayed_messages_read: u64,
}

const BATCH_POSTER_ADDRESS: [u8; 20] = [0u8; 20];
const L2_MESSAGE_KIND: u8 = 3;
const HEADER_LEN: usize = 40;
const BROTLI_MESSAGE_HEADER_BYTE: u8 = 0x00;
const ZEROHEAVY_FLAG: u8 = 0x20;
const BATCH_SEGMENT_KIND_L2_MESSAGE: u8 = 0u8;
const BATCH_SEGMENT_KIND_L2_MESSAGE_BROTLI: u8 = 1u8;
const BATCH_SEGMENT_KIND_DELAYED_MESSAGES: u8 = 2u8;
const BATCH_SEGMENT_KIND_ADVANCE_TIMESTAMP: u8 = 3u8;
const BATCH_SEGMENT_KIND_ADVANCE_L1_BLOCK_NUMBER: u8 = 4u8;

pub fn decode_sequencer_message(sequencer_msg: String) -> Result<Vec<MessageWithMetadata>> {
    // if sequencer_msg.len() < HEADER_LEN + 1 {
    //     bail!("sequencer message is too short");
    // }
    println!("Sequencer message length: {:?}", sequencer_msg);
    let sequencer_msg = &hex::decode(sequencer_msg)?[..];

    let min_timestamp = 1769197492;
    let max_timestamp = 1769287492;
    let min_l1_block = 0;
    let max_l1_block = 46;
    let after_delayed_messages = 2;

    // let mut payload = &sequencer_msg[HEADER_LEN..];
    let mut payload = sequencer_msg;
    let header_byte = payload[0];

    payload = &payload[1..];

    if header_byte & ZEROHEAVY_FLAG != 0 {
        bail!("zeroheavy flag set; add zeroheavy decode here")
    }

    if header_byte != BROTLI_MESSAGE_HEADER_BYTE {
        bail!("unexpected header byte: 0x{:02x}", header_byte);
    }

    let decompressed_batch =
        brotli_decompress(payload).context("brotli decompress sequencer msg")?;

    // The decompressed bytes are a concatenation of RLP-encoded byte slices (segments)
    let mut segments: Vec<Vec<u8>> = Vec::new();
    let mut offset = 0usize;
    while offset < decompressed_batch.len() {
        let info = PayloadInfo::from(&decompressed_batch[offset..])?;
        let end = offset + info.total();
        if end > decompressed_batch.len() {
            break;
        }
        let seg: Vec<u8> = rlp::decode(&decompressed_batch[offset..end])?;
        segments.push(seg);
        offset = end;
    }
    // println!("Segments: {:#?}", segments);

    // Waly through segments now and build messages
    let mut cur_ts = min_timestamp;
    let mut cur_block = min_l1_block;
    let mut delayed_read = 0u64;
    let mut out = Vec::new();

    for seg in segments {
        if seg.is_empty() {
            continue;
        }
        let kind = seg[0];
        let body = &seg[1..];

        match kind {
            // BATCH_SEGMENT_KIND_L2_MESSAGE = 0 && BATCH_SEGMENT_KIND_L2_MESSAGE_BROTLI = 1
            BATCH_SEGMENT_KIND_L2_MESSAGE | BATCH_SEGMENT_KIND_L2_MESSAGE_BROTLI => {
                let mut l2msg = body.to_vec();

                if kind == BATCH_SEGMENT_KIND_L2_MESSAGE_BROTLI {
                    // Message is brotli compressed
                    l2msg =
                        brotli_decompress(body).context("brotli decompress L2 message segment")?;
                }
                let ts = cur_ts.clamp(min_timestamp, max_timestamp);
                let block = cur_block.clamp(min_l1_block, max_l1_block);
                out.push(MessageWithMetadata {
                    message: L1IncomingMessage {
                        header: L1IncomingMessageHeader {
                            kind: L2_MESSAGE_KIND,
                            poster: BATCH_POSTER_ADDRESS,
                            block_number: block,
                            timestamp: ts,
                            l1_base_fee: 0,
                        },
                        l2msg,
                    },
                    delayed_messages_read: delayed_read,
                });
            }
            BATCH_SEGMENT_KIND_DELAYED_MESSAGES => {
                // Delayed message placeholder; just advance the counter.
                delayed_read += 1;
                if delayed_read > after_delayed_messages {
                    // Matches Nitro behavior: log/warn; here we just stop.
                    break;
                }
            }
            BATCH_SEGMENT_KIND_ADVANCE_TIMESTAMP | BATCH_SEGMENT_KIND_ADVANCE_L1_BLOCK_NUMBER => {
                let adv: u64 = Rlp::new(body)
                    .as_val()
                    .context("rlp decode advance value")?;
                if kind == BATCH_SEGMENT_KIND_ADVANCE_TIMESTAMP {
                    cur_ts = cur_ts.saturating_add(adv);
                } else {
                    cur_block = cur_block.saturating_add(adv);
                }
            }
            _ => bail!("unknown segment kind {}", kind),
        }
    }

    Ok(out)
}

fn brotli_decompress(input: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    {
        let mut writer = DecompressorWriter::new(&mut out, 16 * 1024 * 1024); // 16 MiB cap like Nitro
        writer
            .write_all(input)
            .and_then(|_| writer.flush())
            .context("brotli decompressor")?;
    } // writer is dropped here
    Ok(out)
}
