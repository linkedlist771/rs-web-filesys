use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use clap::{App as ClapApp, Arg};
use env_logger::Env;

mod router;

pub struct AppState {
    upload_dir: String,
}

fn create_dir_if_not_exists(dir: &str) {
    if !std::path::Path::new(dir).exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = ClapApp::new("File Server")
        .arg(
            Arg::with_name("host")
                .long("host")
                .default_value("127.0.0.1")
                .help("Host address to bind"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .default_value("8080")
                .help("Port to listen on"),
        )
        .arg(
            Arg::with_name("upload_dir")
                .long("upload-dir")
                .default_value("./data")
                .help("Directory to store uploaded files"),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let upload_dir = matches.value_of("upload_dir").unwrap().to_string();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    create_dir_if_not_exists(&upload_dir);

    println!("Server starting at http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        let logger = Logger::default();

        App::new()
            .app_data(web::Data::new(AppState {
                upload_dir: upload_dir.clone(),
            }))
            .wrap(logger)
            .wrap(cors)
            .configure(router::config)
    })
    .bind((host, port))?
    .run()
    .await
}
