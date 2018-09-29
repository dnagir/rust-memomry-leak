extern crate actix_web;
extern crate env_logger;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use actix_web::fs::StaticFiles;
use actix_web::http::Method;
use actix_web::middleware::cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{server, App, Json, Result};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FileDetails {
    name: String,
    content: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PostedSessionFiles {
    data: Vec<FileDetails>,
}

fn take_data(data: Json<PostedSessionFiles>) -> Result<String> {
    let lens: usize = data.data.iter().map(|file| file.content.len()).sum();
    println!("Total files length: {}chars", lens);
    Ok(lens.to_string())
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new(|| {
        let static_files = StaticFiles::new("./").expect("static files dir not available");

        App::new()
            .configure(|app| {
                Cors::for_app(app)
                    .send_wildcard()
                    .resource("api/take_data", |r| {
                        r.method(Method::POST).with_config(take_data, |cfg| {
                            cfg.0.limit(200 * 1024 * 1024); // 200 MB; default - 256kB
                            ()
                        })
                    })
                    .register()
            })
            .middleware(Logger::default())
            .handler("/", static_files)
    }).bind("0.0.0.0:7000")
        .expect("cannot bind to the port")
        .run();
}
