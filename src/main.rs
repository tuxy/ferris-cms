use std::{ fs, collections::HashMap };

use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod parse;
mod pages;

#[derive(Default, Debug, Clone)]
struct Cache {
    cache: HashMap<String, String>
}

impl Cache {

    // Not very performant but is a usable and simple cache
    fn cache_pages() -> Cache {
        let mut state = Cache {
            cache: HashMap::new()
        };

        let paths = fs::read_dir("dist").unwrap();
        for path in paths {
            let path = path.unwrap().path().clone();

            let file = match fs::read_to_string(path.display().to_string()) {
                Ok(val) => val,
                Err(_) => fs::read_to_string("dist/404.md")
                    .expect("404.md doesn't exist")
            };

            state.cache.insert(
                String::from(path.file_name().unwrap().to_str().unwrap()),
                file
            );
        }
        state
    }
}

#[tokio::main]
async fn main() {
    let state = Cache::cache_pages();

    let app = Router::new()
        .route("/{path}", get(pages::adaptive_page).with_state(state));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
