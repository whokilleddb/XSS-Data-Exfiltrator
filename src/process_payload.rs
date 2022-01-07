use crate::convert;
use std::io::Write;
use std::fs;

// Function To Print Cookie
pub fn decode_payload(key: &str, value: &str, logfile: String, verbose: bool){
    if verbose {
        let payload = format!("[+] Encoded {}: \n{}", key, value);
        println!("{}",payload);
        write_to_file(logfile.as_str(), payload.as_str());
    };
    
    let decoded = convert::decoder(value);
    let payload = format!("\n[+] Decoded {}: \n{}", key, decoded);
    println!("[+] Decoded {}: \n{}",key, decoded);
    write_to_file(logfile.as_str(), payload.as_str());
}

fn write_to_file(filename: &str, payload: &str){
    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .create(true)
      .open(filename)
      .unwrap(); 
      //{
       // Ok(ptr) => ptr,
      //  Err(err) => convert::exit_on_error("Could Not Create Logfile"),
   // };

    write!(file, "{}", payload).expect("Could Not Write To File!");
}