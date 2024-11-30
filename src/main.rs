use std::fs;
use serde::Deserialize;

use tiny_http::{Server, Response};

mod parse;

#[derive(Deserialize)]
struct Config {
    bind_address: String,
    custom_css: String,
    bar: Bar,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Bar {
    names: Vec<String>,
    urls: Vec<String>,
}

fn main() {

    // Opens config.toml from root
    // TODO: Fix unwraps. Also wtf is the chain lmao
    let config: Config = toml::from_str(
        fs::read_to_string("config.toml").unwrap().as_str()
    ).unwrap();
    println!("{:?}", config.bar);

    let server = Server::http(config.bind_address.as_str()).expect("Could not bind to address.");

    for request in server.incoming_requests() {

        // Get and parse request.url(), which returns something like /test
        // Reads from .md file
        let filename = match request.url() {
            "/" => "/index",
            _ => request.url(),
        };

        // Parses the filename for the server. More realistic and integrated way to do this?
        let filename = {
            let mut directory = String::from("dist");
            directory.push_str(&filename);
            directory.push_str(".md");
            directory
        };

        // If page not found. TODO add reason
        let content = match fs::read_to_string(filename) {
            Ok(val) => val,
            Err(_) => fs::read_to_string("dist/404.md").expect("Error reading files. Is 404.md there?"),
        };

        let html = parse::parse(&content, request.url(), &config.custom_css);

        let response = Response::from_data(html);

        request.respond(response).unwrap();
    }
}
