use std::fs;
use markdown;
use html_escape;
use serde::Deserialize;

use tiny_http::{Server, Response};

#[derive(Deserialize)]
struct Config {
    bind_url: String,
    custom_css: String,
}

fn main() {

    // Opens config.toml from root
    // TODO: Fix unwraps. Also wtf is the chain lmao
    let config: Config = toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap();

    let server = Server::http(config.bind_url.as_str()).expect("Could not bind to address.");

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

        // Converts to html
        let html = markdown::to_html(&content);
        let mut html_decoded = String::new();

        // Uses a LIBRARY to remove html escapes. PLS change it sucks
        // unsafe { from_utf8_unchecked(decode_html_entities_to_vec(text, output.as_mut_vec())) }
        // UP THERE is the unsafe version. We could use it like that, but lets see...
        html_escape::decode_html_entities_to_string(html, &mut html_decoded);

        // Adding extra styles code to the beginning, with <head> (Maybe possible for Adding titles as well????)
        // Something like new.css or simple.css would be amazing here
        let mut html = String::from(format!("<head>{}<head>", config.custom_css.clone().as_str()));
        html.push_str(&html_decoded);

        let response = Response::from_data(html);

        request.respond(response).unwrap();
    }
}