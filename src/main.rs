use std::fs;
use markdown;
use html_escape;

use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("127.0.0.1:7878").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let content = fs::read_to_string("dist/index.md").unwrap();
        let html = markdown::to_html(&content);
        //println!("{html}");
        let mut html_decoded = String::new();
        html_escape::decode_html_entities_to_string(html, &mut html_decoded);
        let response = Response::from_data(html_decoded);
        let _ = request.respond(response);
    }

}