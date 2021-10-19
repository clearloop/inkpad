//! Crypto fns
use crate::Sandbox;

impl Sandbox {
    /// Verify and recover a SECP256k1 ECDSA signature.
    ///
    /// - `sig` is passed in RSV format. V should be either `0/1` or `27/28`.
    /// - `msg` is the blake2-256 hash of the message.
    ///
    /// Returns `Err` if the signature is bad, otherwise the 33-byte compressed pubkey.
    #[allow(clippy::result_unit_err)]
    pub fn ecdsa_recover(&self, sig: &[u8; 65], msg: &[u8; 32]) -> Result<[u8; 33], ()> {
        let rs = libsecp256k1::Signature::parse_overflowing_slice(&sig[0..64]).map_err(|_| ())?;
        let v = libsecp256k1::RecoveryId::parse(
            if sig[64] > 26 { sig[64] - 27 } else { sig[64] } as u8
        )
        .map_err(|_| ())?;
        let pubkey =
            libsecp256k1::recover(&libsecp256k1::Message::parse(msg), &rs, &v).map_err(|_| ())?;
        Ok(pubkey.serialize_compressed())
    }
}
