use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    elliptic_curve::Error,
    FieldBytes, SecretKey,
};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Inputs {
    pub x: Vec<u8>,
    pub y: Vec<u8>,
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
}

pub struct InputGenerator {
    // where sk is the signing key
    pub sk: SigningKey,
    pub message: Vec<u8>,
}

impl InputGenerator {
    pub fn generate(&self) -> Inputs {
        // hash the message using a fresh instance of sha2
        let mut sha_256 = Sha256::new();
        sha_256.update(&self.message);
        let message_hash = sha_256.finalize();
        // generate a signature for the hashed message
        let signature: Signature = self.sk.sign(&message_hash);
        // derive the x and y coordinate of the verifying key
        let verifying_key = VerifyingKey::from(&self.sk).to_encoded_point(false);
        Inputs {
            x: verifying_key.x().unwrap().as_slice().to_vec(),
            y: verifying_key.y().unwrap().as_slice().to_vec(),
            message: message_hash.to_vec(),
            signature: signature.to_vec(),
        }
    }
}
