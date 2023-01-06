#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;
use std::fs::{read_to_string, read};
use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;


enum FileTypes {
    ByteVector(Vec<u8>),
    String(String)
    
}

lazy_static! {
    static ref IS_FILE_REGEX: Regex = Regex::new(r"(\S+)\.(\S+)").unwrap();
}


fn get_extension_from_filename(filename: &str) -> Option<&str> {    
    Path::new(filename)        
    .extension()        
    .and_then(OsStr::to_str)
}


async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, path) if IS_FILE_REGEX.is_match(path) => {
            println!("path: {}", path);
            let file: FileTypes;
            let mut status = StatusCode::OK;
            let mut content_header = "text/html";
            let file_ext;
            
            match Path::new(path).try_exists() {
                Ok(y) => {
                    match y {
                        true => {
                            file_ext = get_extension_from_filename(path).unwrap();
                            file = match file_ext {
                                "wasm" => FileTypes::ByteVector(read(path).unwrap()),
                                _ => FileTypes::String(read_to_string(path).unwrap())
                            };
                            content_header = get_content_header(file_ext);
                        },
                        false => {
                            file = FileTypes::String(String::from("File not found"));
                            status = StatusCode::NOT_FOUND;
                        }
                    };
                },
                Err(_) => {
                    status = StatusCode::INTERNAL_SERVER_ERROR;
                    file = FileTypes::String(String::from("Internal server error"));
                }
            }

            fn get_content_header(file_ext: &str) -> &str {
                match file_ext.to_lowercase().as_str() {
                    "js" => "text/javascript",
                    "html" => "text/html",
                    "wasm" => "application/wasm",
                    "css" => "text/css",
                    "md" => "text/markdown",
                    "ttf" => "font/ttf",
                    "otf" => "font/otf",
                    "woff" => "font/woff",
                    "woff2" => "font/woff2",
                    "sfnt" => "font/sfnt",
                    "rs" => "test/plain",
                    "toml" => "test/plain",
                    _ => "application/octet-stream"
                }
            }

            Ok(Response::builder()
                .header("Content-type", content_header)
                .status(status)
                .body(match file {
                    FileTypes::ByteVector(x) => Body::from(x),
                    FileTypes::String(x) => Body::from(x)
                }).unwrap()
            )
        },
        (&Method::GET, "/") => Ok(
            Response::new(
                Body::from("Hello world from Rust running with Wasm! Send POST data to /echo to have it echoed back to you")
            )
        ),
        (&Method::GET, "/index") => Ok(
            Response::new(
                Body::from(
                    read_to_string("/files/hello_world/index.html")
                    .expect("Should be able to read index.html")
                )
            )
        ),
        (&Method::GET, "/js") => {
            Ok(Response::builder()
                .header("Content-type", "application/javascript")
                .body(Body::from(read_to_string("/files/index.js").expect("Should be able to read index.js")))
                .expect("Should be able to create JS response")
            )
        },
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port https://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
