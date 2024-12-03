use markdown::{ self, Options };
use html_escape;

pub fn parse(content: &str, url: &str, css: &str) -> String {

    let gfm: Options = Options::gfm(); // Default GitHub flavoured markdown settings

    let html = markdown::to_html_with_options(content, &gfm).unwrap();
    let mut html_decoded = String::new(); // Will be appended to later

    // Not a great or efficient way to fix escape
    html_escape::decode_html_entities_to_string(html, &mut html_decoded);

    // Main parser
    let mut html = String::from(
        format!(
            "<head><link rel='preload' href='{}' as='style'/><link rel='stylesheet' href='{}'><title>{}</title><head>"
            , css, css, url) // Loads into a css url (Could be a file), preloads it and also adds simple titles
    );
    html.push_str(&html_decoded);

    html
}