use tiny_http::Request;
use std::fs;

use crate::config::Config;

pub fn custom_markdown(request: &Request, config: &Config) -> String {
    // Get and parse request.url(), which returns something like /test
        // Reads from .md file
        let filename = match request.url() {
            "/" => "/index",
            _ => request.url(),
        };

        // Parses the filename for the server. More realistic and integrated way to do this?
        let filename = {
            let mut directory = String::from("dist");
            directory.push_str(filename);
            directory.push_str(".md");
            directory
        };

        let mut bar_contents = String::new();
        // Checks if bar table? in the config is populated.
        if let Some(ref val) = config.bar {
            let item_names = &val.names;
            let item_url = &val.urls;
            for (idx, i) in item_names.iter().enumerate() {
                bar_contents.push_str(
                    format!("/ [{}]({}) ", i, item_url[idx]).as_str()
                );
            }
        }
        // Acts as both a spacer for the top of the page and to space the navigation bar away from content.
        bar_contents.push('\n');

        // If page not found. TODO add reason
        let content = match fs::read_to_string(filename) {
            Ok(val) => val,
            Err(_) => fs::read_to_string("dist/404.md").expect("Error reading 404.md. Is 404.md there?"),
        };

        bar_contents.push_str(content.as_str());

        bar_contents
}