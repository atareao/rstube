use actix_web::{HttpServer, App, web::{self, Data}, middleware::Logger};
use std::process;
use tokio::fs;
use env_logger::Env;
use log::{debug, error};
use tera::Tera;
use actix_web_httpauth::extractors::basic;
use actix_files;

use crate::config::Configuration;

mod routes;
mod config;
mod ytdlp;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let content = match fs::read_to_string("config.yml")
        .await {
            Ok(value) => value,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        };
    let configuration = Configuration::new(&content)
        .expect("Someting went wrong");

    let log_level = configuration.get_log_level();
    debug!("Log level: {}", log_level);
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    let port = configuration.get_port();
    debug!("Port: {}", port);

    let template = match Tera::new("templates/**/*.html"){
        Ok(t) => t,
        Err(e) => {
            error!("Can not load templates, {}", e);
            process::exit(1);
        }
    };
    debug!("{:?}", template);

    let conf = configuration.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(basic::Config::default().realm("Restricted area"))
            .wrap(Logger::default())
            .app_data(Data::new(conf.clone()))
            .app_data(Data::new(template.clone()))
            //.service(routes::get_form)
            //.service(routes::post_form)
            .service(routes::index)
            .service(routes::download)
            .service(actix_files::Files::new("/static", "./static"))
    })
    .workers(4)
    .bind(format!("0.0.0.0:{}", &port))
    .unwrap()
    .run()
    .await
}
