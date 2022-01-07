extern crate clap;

use termion::color;
use ctrlc;
use std::process;
use clap::{Arg, App};


mod capture;
//mod convert;
//mod process_payload;

fn main(){
    // Handle CTRL-C 
    ctrlc::set_handler(move || {
        println!("\n{}[!] Received Ctrl+C!{}",color::Fg(color::Red), color::Fg(color::Reset));
        process::exit(0);
    }).expect("[-] Error setting Ctrl-C handler");


    // Command Line Arguments
    let matches = App::new("XSS Information Exfiltrator")
                            .version("0.1.0")
                            .author("@whokilleddb")
                            .about("Exfiltrate Data Via XSS")
                            .arg(Arg::new("port")
                                .short('p')
                                .long("port")
                                .help("Port To Bind To")
                                .takes_value(true))
                            .arg(Arg::new("output")
                                .short('o')
                                .long("output")
                                .help("Name of capture file")
                                .takes_value(true))
                            .arg(Arg::new("verbose")
                                .short('v')
                                .long("verbose")
                                .help("Sets the level of verbosity"))
                            .get_matches();
    
    // Set Filename
    let capture_file = matches.value_of("output").unwrap_or("Capture.log").to_string();
    
    // Set Port
    let port: u16 = matches.value_of("port")
                            .unwrap_or("6969")
                            .trim()
                            .parse()
                            .expect("Invalid Port Number!");

    // Print Message
    println!("{}[+] XSS Exfiltration By @whokilleddb{}", color::Fg(color::Green), color::Fg(color::Reset));
    let verbosity =  if matches.is_present("verbose"){
        true
    } else {
        false 
    };

    match capture::start_listener(port, capture_file, verbosity){
        Err(e) => println!("{}[-] Error Occured As: {:?}{}",color::Fg(color::Red), e, color::Fg(color::Reset)),
        _ => ()
    };  
}