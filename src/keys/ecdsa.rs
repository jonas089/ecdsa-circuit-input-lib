use std::path::PathBuf;
use k256::{
    ecdsa::{SigningKey}, FieldBytes
};
use rand_core::OsRng;

pub struct EcdsaKeyManager{
    pub slice: Vec<u8>
}

impl EcdsaKeyManager{
    pub fn new(self) -> Vec<u8>{
        let signing_key = SigningKey::random(&mut OsRng);
        let signing_key_bytes_generic = signing_key.to_bytes();
        let field_bytes_slice = signing_key_bytes_generic.as_slice();
        /*
        let field = FieldBytes::from_slice(field_bytes_slice);
        let signing_key_deserialized = SigningKey::from_bytes(&signing_key_bytes_generic).unwrap();
        assert_eq!(signing_key, signing_key_deserialized);
        */

        field_bytes_slice.to_vec()
    }
    
    pub fn deserialize(self) -> SigningKey{
        let field = FieldBytes::from_slice(&self.slice);
        SigningKey::from_bytes(&field)
            .expect("[Error] Failed to deserialize secret key!")
    }
}