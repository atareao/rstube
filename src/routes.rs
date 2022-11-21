use actix_web::{get, post, web, Result, error, Error, Responder};
use actix_web_lab::respond::Html;
use tera::Tera;
use log::debug;
use actix_web_httpauth::extractors::basic::BasicAuth;
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
        template.render("index.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))
    };
    let html_content = html.unwrap();
    debug!("Content: {}", &html_content);
    Ok(Html(html_content))
}
