use std::fs;

use axum::{ extract::{Path, State}, response::Html };
use crate::{parse::Parser, Cache};

pub async fn adaptive_page(Path(path): Path<String>, State(state): State<Cache>) -> Html<String> {

    let path = path + ".md";

    let markdown = state.cache.get(&path).unwrap();
    
    let parser = Parser {
        content: markdown.to_string(),
        path: path,
        css: String::from("https://cdn.simplecss.org/simple.css")
    };

    Html(parser.parse())
}