use tiny_http::{Server, Response};

mod parse;
mod config;
mod bar;

fn main() {

    // Opens config.toml from root
    let config: config::Config = config::open_config();

    let server = Server::http(config.bind_address.as_str())
        .expect("Could not bind to address.");
    
    println!("Bind address: {}", config.bind_address);

    for request in server.incoming_requests() {

        let content= bar::custom_markdown(&request, &config);
        // Converts to html with options

        let html = parse::parse(&content, request.url(), &config.custom_css);

        let response = Response::from_data(html);

        match request.respond(response) {
            Ok(_) => (),
            Err(_) => panic!("Could not respond to request")
        };
    }
}
