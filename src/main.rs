use std::fs;
use markdown;
use html_escape;

use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("127.0.0.1:7878").expect("Could not bind to address.");

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
        let response = Response::from_data(html_decoded);

        // Why does this suck
        let _ = request.respond(response);
    }
}