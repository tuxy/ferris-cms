use std::fs;
use tiny_http::{Server, Response};

mod parse;
mod config;

fn main() {

    // Opens config.toml from root
    let config: config::Config = config::open_config();

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

        let html = parse::parse(&bar_contents, request.url(), &config.custom_css);

        let response = Response::from_data(html);

        match request.respond(response) {
            Ok(_) => (),
            Err(_) => panic!("Could not respond to request")
        };
    }
}
