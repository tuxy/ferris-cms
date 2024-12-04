use std::fs;
use serde::Deserialize;

use tiny_http::{Server, Response};

mod parse;

#[derive(Deserialize)]
struct Config {
    bind_address: String,
    custom_css: String,
    bar: Option<Bar>,
}

#[derive(Deserialize)]
#[derive(Clone)]
struct Bar {
    names: Vec<String>,
    urls: Vec<String>,
}

fn main() {

    // Opens config.toml from root
    let config: Config = match toml::from_str::<Result<Config, ()>>(fs::read_to_string("config.toml").unwrap().as_str()) {
        // Handle config parse error case
        Ok(parse_result) => {
            match parse_result {
                // Handle readable config case
                Ok(res) => res,
                Err(_) => panic!("Could not read config. Check if file exists and is readable?")
            }
        },
        Err(_) => panic!("Could not parse config. Check format?")
    };

    let server = Server::http(config.bind_address.as_str())
        .expect("Could not bind to address.");

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

        let mut bar_contents = String::new();
        // Checks if bar table? in the config is populated.
        match config.bar {
            Some(ref val) => {
                let item_names = &val.names;
                let item_url = &val.urls;
                for (idx, i) in item_names.iter().enumerate() {
                    bar_contents.push_str(
                        format!("/ [{}]({}) ", i, item_url[idx]).as_str()
                    );
                }
            },
            None => (),
        }
        bar_contents.push_str("\n");

        // If page not found. TODO add reason
        let content = match fs::read_to_string(filename) {
            Ok(val) => val,
            Err(_) => fs::read_to_string("dist/404.md").expect("Error reading 404.md. Is 404.md there?"),
        };

        bar_contents.push_str(&content.as_str());
        // Converts to html with options

        let gfm = Options::gfm(); // Default GitHub flavoured markdown settings

        let html = match markdown::to_html_with_options(&content, &gfm) {
            // Handle markdown parse case 
            Ok(val) => val,
            Err(_) => panic!("Could not parse markdown file")
        };

        let mut html_decoded = String::new();

        // Uses a LIBRARY to remove html escapes. PLS change it sucks
        // unsafe { from_utf8_unchecked(decode_html_entities_to_vec(text, output.as_mut_vec())) }
        // UP THERE is the unsafe version. We could use it like that, but lets see...
        html_escape::decode_html_entities_to_string(html, &mut html_decoded);

        let html = parse::parse(&bar_contents, request.url(), &config.custom_css);

        let response = Response::from_data(html);

        match request.respond(response) {
            Ok(_) => (),
            Err(_) => panic!("Could not respond to request")
        };
    }
}
