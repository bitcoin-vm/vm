use super::transfer::Transfer;
use crate::{
    hash::{tagged_hash, HashTag},
    serialization::{cpe::CompactPayloadEncoding, serialize::Serialize, sighash::Sighash},
    signature::schnorr::{schnorr_sign, SecpError, SignEntry, SignFlag},
};
use bit_vec::BitVec;

pub enum Entry {
    Transfer(Transfer),
}

impl CompactPayloadEncoding for Entry {
    fn to_cpe(&self) -> BitVec {
        match self {
            Entry::Transfer(transfer) => transfer.to_cpe(),
        }
    }
}

impl Sighash for Entry {
    fn sighash(&self, prev_state_hash: [u8; 32]) -> [u8; 32] {
        let mut sighash_preimage = Vec::<u8>::new();

        sighash_preimage.extend(prev_state_hash);

        let (serialized_entry, sighash_tag) = match self {
            Entry::Transfer(transfer) => (transfer.serialize(), HashTag::SighashTransfer),
        };

        sighash_preimage.extend(serialized_entry);

        tagged_hash(sighash_preimage, sighash_tag)
    }
}

impl SignEntry for Entry {
    fn sign(&self, secret_key: [u8; 32], prev_state_hash: [u8; 32]) -> Result<[u8; 64], SecpError> {
        // Message is the sighash of the entry.
        let message = self.sighash(prev_state_hash);

        // Sign the message with the entry signing method.
        schnorr_sign(secret_key, message, SignFlag::EntrySign)
    }
}