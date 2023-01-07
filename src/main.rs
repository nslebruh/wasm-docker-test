#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;
use std::fs::read;
use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;


lazy_static! {
    static ref IS_FILE_REGEX: Regex = Regex::new(r"(\S+)\.(\S+)").unwrap();
}


fn get_extension_from_filename(filename: &str) -> Option<&str> {    
    Path::new(filename)        
    .extension()        
    .and_then(OsStr::to_str)
}


async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();
    if req.method() == Method::GET {
        if IS_FILE_REGEX.is_match(path) {
            println!("path: {}", path);
            let file: Option<Vec<u8>>;
            let mut status = StatusCode::OK;
            let mut content_header = "text/plain";
            
            match Path::new(path).try_exists() {
                Ok(y) => {
                    match y {
                        true => {
                            let file_ext = get_extension_from_filename(path).unwrap();
                            file = Some(read(path).unwrap());
                            content_header = get_content_header(file_ext);
                        },
                        false => {
                            status = StatusCode::NOT_FOUND;
                            file = None;
                        }
                    };
                },
                Err(_) => {
                    status = StatusCode::INTERNAL_SERVER_ERROR;
                    file = None;
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
                    "rs" => "text/plain",
                    "toml" => "text/plain",
                    _ => "application/octet-stream"
                }
            }
            
            return Ok(Response::builder()
                .header("Content-type", content_header)
                .status(status)
                .body(match file {
                    Some(x) => Body::from(x),
                    None => Body::empty()
                }).unwrap()
            )
        };
    }
    match path {
        "/" => Ok(
            Response::new(
                Body::from("Hello world from Rust running with Wasm! Send POST data to /echo to have it echoed back to you")
            )
        ),
        "/index" => Ok(
            Response::new(
                Body::from(
                    read("/files/hello_world/index.html")
                    .expect("Should be able to read index.html")
                )
            )
        ),
        "/echo" => Ok(Response::new(req.into_body())),
        _ => Ok(
            Response::builder()
                .status(404)
                .body(
                    Body::from("Path not found")
                ).unwrap()
            )
    }
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port https://localhost:{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(router)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
