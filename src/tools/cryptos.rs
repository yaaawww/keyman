use rand::{distributions::Alphanumeric, Rng, rngs::OsRng};
use crate::{KEY, ApiError};

use crypto::aes::{self, KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;

pub fn generate_cipher(password: String) -> Result<Vec<String>, ApiError>{
    let mut aes_key: &[u8; 16] = &[0u8; 16];
    unsafe {
        match &KEY {
            Some(key) => aes_key = *key,
            _ => return Err(ApiError::MasterKeyMissing),
        }
    }

    // generate the vi TODO
    // fxxk error handle
    let iv = [42u8; 16];
    //let file = std::fs::File::create("VectorFile").expect("create failed");
    //write(
        //"VectorFile",
        //base64::encode(&vi)
    //).unwrap(); 

    // mode key vi

    let mut cipher = aes::ctr(KeySize::KeySize128, aes_key, &iv);
    let mut output: Vec<u8> = std::iter::repeat(0u8).take(password.len()).collect();
    cipher.process(password.as_bytes(), &mut output[..]);
    // 0 iv, 1 cipher
    Ok(vec![base64::encode(&iv), base64::encode(&output)])
}

pub fn decrypt(pwd_str: String, iv_str: String) -> Result<String, ApiError> {
    let mut aes_key: &[u8; 16] = &[0u8; 16];
    unsafe {
        match &KEY {
            Some(key) => aes_key = *key,
            _ => return Err(ApiError::MasterKeyMissing),
        }
    }

    let pwd = base64::decode(&pwd_str).unwrap();
    let iv = base64::decode(&iv_str).unwrap();

    let mut cipher = aes::ctr(KeySize::KeySize128, aes_key, &iv);
    let mut output: Vec<u8> = std::iter::repeat(0u8).take(pwd.len()).collect();
    cipher.process(&pwd, &mut output[..]);
    // 0 iv, 1 cipher
    Ok(std::str::from_utf8(&output).unwrap().to_string())
}

// generate a random password
pub fn generate_plain_key() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    s
}

//pub fn recover_key() -> String {

 