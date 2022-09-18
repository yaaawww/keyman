#![allow(unused)]
#[test]
pub fn test_num_to_char() {
    let key:Vec<u8> = vec![148, 38, 92, 83];
    let key_string = base64::encode(&key);
    println!("{}", base64::encode(&key));
    println!("{:?}", base64::decode(key_string));
    println!("{:?}", key);
}

pub fn num_to_str(num: u8) {

}
