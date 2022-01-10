extern crate termion;
extern crate hex;
extern crate base64;

use termion::color;
use std::process;
use std::str;

pub fn exit_on_error(msg: &str) -> !{ 
    eprintln!("{}[-] Error: {}{}", color::Fg(color::Red), msg,  color::Fg(color::Reset));
    process::exit(-1);
}

pub fn vec_u8_to_str<'a>(vec:&'a  Vec<u8>) -> &'a str {
    let decoded: &str = match str::from_utf8(&vec){
        Ok(num) => num,
        Err(_) => exit_on_error("Could Not Convert Vec<u8> slice to string"),
    };
    return decoded;   
}


pub fn decoder<'a>(enc: &'a str) -> String{
    // Decode &str to Vec<u8>
    let hexdecoded = match hex::decode(enc){
        Ok(num) => num,
        Err(_) => exit_on_error("Could Not Decode Hex :("),
    };

    let base64_enc = match base64::decode(vec_u8_to_str(&hexdecoded)){
        Ok(num) => num,
        Err(_) => exit_on_error("Could Not Decode Base64 :("),
    };

    let decoded_message: &str = vec_u8_to_str(&base64_enc);
    return decoded_message.to_string();
}
