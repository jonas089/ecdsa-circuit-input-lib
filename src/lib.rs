pub mod keys;
pub mod db;
pub mod core;

#[test]
fn store_key(){
    use std::path::PathBuf;
    use db::StoreManager;
    use keys::ecdsa::EcdsaKeyManager;
    // initialize key manager
    let key_manager = EcdsaKeyManager{
        slice: vec![]
    };
    // generate new key
    let key_serialized = key_manager.new();
    // initialize keystore
    let store_manager = StoreManager{
        path: PathBuf::from("./keys.db")
    };
    store_manager.init()
        .expect("[Error] Failed to initialize keystore");
    // store serialized key
    store_manager.insert_key(
        "SOME_KEY_UID_0".to_string(), 
        key_serialized)
            .expect("[Error] Failed to store key!");
}

#[test]
fn get_keys(){
    use std::path::PathBuf;
    use db::StoreManager;
    use keys::ecdsa::EcdsaKeyManager;
    // initialize keystore
    let store_manager = StoreManager{
        path: PathBuf::from("./keys.db")
    };
    // get keys
    let keys = store_manager
        .get_keys()
        .expect("[Error] Failed to get keys!");
    println!("Keys: {:?}", &keys);
}

#[test]
fn deserialize_key(){
    use std::path::PathBuf;
    use db::StoreManager;
    use keys::ecdsa::EcdsaKeyManager;
    use k256::{
        ecdsa::{SigningKey}, FieldBytes
    };
    // initialize keystore
    let store_manager: StoreManager = StoreManager{
        path: PathBuf::from("./keys.db")
    };
    // get key
    let key: db::Response = store_manager
        .get_key_by_uid("SOME_KEY_UID_0".to_string())
        .expect("[Error] Failed to get the key!");
    println!("Key: {:?}", &key);
    // deserialize SigningKey from Response object
    let key_slice: Vec<u8> = key.deserialize();
    let key_manager: EcdsaKeyManager = EcdsaKeyManager{
        slice: key_slice
    };
    // signing key ready for use with input generator
    let deserialized_signing_key = key_manager.deserialize();
}

#[test]
fn generate_signature_circuit_inputs(){
    use std::path::PathBuf;
    use db::StoreManager;
    use keys::ecdsa::EcdsaKeyManager;
    use crate::core::signatures::InputGenerator;
    use k256::{
        ecdsa::{SigningKey}, FieldBytes
    };
    // initialize keystore
    let store_manager: StoreManager = StoreManager{
        path: PathBuf::from("./keys.db")
    };
    // get key
    let key: db::Response = store_manager
        .get_key_by_uid("SOME_KEY_UID_0".to_string())
        .expect("[Error] Failed to get the key!");
    // deserialize SigningKey from Response object
    let key_slice: Vec<u8> = key.deserialize();
    let key_manager: EcdsaKeyManager = EcdsaKeyManager{
        slice: key_slice
    };
    // signing key ready for use with input generator
    let deserialized_signing_key = key_manager.deserialize();
    /* 
        * define a message that is to be signed
        * a message can be a serialized struct
        * an example for a message struct would be a blockchain transfer/transaction/deploy
        #[derive(Serialize, Deserialize)]
        struct Transfer{
            sender: 
            recipient:
            amount:
            signature:
            nonce:
            timestamp:
            ...
        }
    */
    // for this test, a placeholder message will be used
    let message: Vec<u8> = vec![0;32];
    // initialize the input generator
    let input_generator = InputGenerator{
        sk: deserialized_signing_key,
        message: message
    };
    let inputs = input_generator.generate();
    println!("Circuit Inputs: {:?}", inputs);
}