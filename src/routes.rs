use std::collections::HashMap;

use actix_web::{get, post, web, Result, error, Error, Responder};
use actix_web_lab::respond::Html;
use tera::Tera;
use log::debug;
use actix_web_httpauth::extractors::basic::BasicAuth;
use regex::Regex;
use crate::config::Configuration;

#[get("/")]
pub async fn index(auth: BasicAuth, template: web::Data<Tera>,
        configuration: web::Data<Configuration>) -> Result<impl Responder, Error>{
    let html = if !configuration.check_basic_auth(&auth) {
        debug!("Not authenticated: {}", &auth.user_id());
        let mut ctx = tera::Context::new();
        ctx.insert("title", "401 Unauthorized");
        template.render("error.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))
    }else{
        debug!("Authenticated: {}", &auth.user_id());
        let mut ctx = tera::Context::new();
        ctx.insert("title", "Title");
        ctx.insert("instructions", "Instructions");
        template.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))
    };
    match html {
        Ok(content) => {
            debug!("Content: {}", &content);
            Ok(Html(content))
        },
        Err(e) => Err(e)
    }
}

#[post("/")]
pub async fn download(auth: BasicAuth, template: web::Data<Tera>,
        configuration: web::Data<Configuration>,
        content: web::Form<HashMap<String, String>>)
        -> Result<impl Responder, Error>{
    let html = if !configuration.check_basic_auth(&auth) {
        debug!("Not authenticated: {}", &auth.user_id());
        let mut ctx = tera::Context::new();
        ctx.insert("title", "401 Unauthorized");
        template.render("error.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))
    }else{
        debug!("Authenticated: {}", &auth.user_id());
        debug!("{:?}", content);
        let yturl = content.get("yturl").unwrap();
        debug!("Respuesta: {}", yturl);
        match  get_yt_id2(&yturl) {
            Some(value) => debug!("Resultado: {}", value),
            None => debug!("Ningún resultado"),
        }
        debug!("Id: {}", yturl);
        let mut ctx = tera::Context::new();
        ctx.insert("title", "Title");
        ctx.insert("instructions", "Instructions");
        template.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))
    };
    let html_content = html.unwrap();
    debug!("Content: {}", &html_content);
    Ok(Html(html_content))
}

fn get_yt_id(url: &str) -> Option<String>{
    let rgx = Regex::new(r"(?i)(youtube\.com|youtu\.be|youtube-nocookie\.com)\\/(?:embed\\/|v\\/|watch\?v=|watch\?list=(.*)&v=)?((\w|-){11})(&list=(\w+)&?)?")
        .unwrap();
    debug!("{:?}", rgx.captures(url));
    match rgx.captures(url){
        Some(capture) => Some(capture[3].to_string()),
        None => None,
    }
}

fn get_yt_id2(url: &str) -> Option<String>{
    let parts = url.split("v=").collect::<Vec<&str>>();
    debug!("{:?}", parts);
    if parts.len() > 1{
        let result = parts.get(1).unwrap();
        debug!("{:?}", result);
        match result.find("&"){
            Some(v) => Some(result[..v].to_string()),
            None => None,
        }
    }else{
        None
    }
}
