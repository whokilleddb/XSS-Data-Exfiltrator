use crate::convert;
use std::io::Write;
use std::fs;
use termion::color;
use std::process;

// Function To Print Cookie
pub fn decode_payload(value: &str, logfile: String){
    println!("{}-----------------{}", color::Fg(color::Red),color::Fg(color::Reset));
    let mut encoded_filename: String = logfile.clone();
    encoded_filename.insert_str(0, "Encoded-"); 
    write_to_file(encoded_filename, value);
    println!("{}[+] Encoded Payload Successfully Written To: Encoded-{}{}", color::Fg(color::Yellow),logfile,color::Fg(color::Reset));
    
    let decoded = convert::decoder(value);
    println!("[+] Decoded Payload: \n{}", decoded);
    let mut decoded_filename: String = logfile.clone();
    decoded_filename.insert_str(0, "Decoded-"); 
    write_to_file(decoded_filename, decoded.as_str());
    println!("{}[+] Encoded Payload Successfully Written To: Decoded-{}{}", color::Fg(color::Cyan),logfile,color::Fg(color::Reset));
    println!("{}[+] Exiting {}🏳️", color::Fg(color::Magenta),color::Fg(color::Reset));
    process::exit(0);    
}

fn write_to_file(filename: String, payload: &str){
    println!("{}[+] Writing Data To: {}{}", color::Fg(color::Yellow), filename, color::Fg(color::Reset) );
    let mut file = fs::OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(filename.as_str())
      .unwrap(); 
      //{
       // Ok(ptr) => ptr,
      //  Err(err) => convert::exit_on_error("Could Not Create Logfile"),
   // };

    write!(file, "{}", payload).expect("Could Not Write To File!");
}