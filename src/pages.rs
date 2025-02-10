use std::fs;

use axum::{ extract::Path, response::Html };
use crate::parse::Parser;

pub async fn adaptive_page(Path(path): Path<String>) -> Html<String> {
    let markdown = load_page(&path);
    let parser = Parser {
        content: markdown,
        path: path,
        css: String::from("https://cdn.simplecss.org/simple.css")
    };

    Html(parser.parse())
}

fn load_page(path: &str) -> String {
    // Get and parse request.url(), which returns something like /test
    // Reads from .md file
    let filename = format!("/{path}");

    // Parses the filename for the server. More realistic and integrated way to do this?
    let filename = {
        let mut directory = String::from("dist");
        directory.push_str(&filename);
        directory.push_str(".md");
        directory
    };

    // Return 404 if doesn't exist
    return match fs::read_to_string(filename) {
        Ok(val) => val,
        Err(_) => fs::read_to_string("dist/404.md").expect("Error reading 404.md. Is 404.md there?"),
    };
}