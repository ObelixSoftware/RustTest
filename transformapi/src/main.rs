use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod html_changer;
mod enums;
use html_changer::HtmlChanger;
use enums::TransformType;

#[derive(Deserialize)]
struct TransformRequest {
    transform: String,
    html: String,
}

#[post("/transform")]
async fn transform(request: web::Json<TransformRequest>) -> impl Responder {
    if request.transform.trim().is_empty() || request.html.trim().is_empty() {
        return HttpResponse::BadRequest()
            .body("Both 'transform' and 'html' must be provided and non-empty");
    }
    
    let transform_type = match TransformType::from_str(&request.transform) {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("Invalid transform type. Must be 'uppercase' or 'lowercase'")
    };

    match HtmlChanger::transform(transform_type, request.html.clone()) {
        Ok(transformed_html) => {
            HttpResponse::Ok()
            .content_type("text/plain")
            .body(transformed_html)
        },
        Err(_e) => {
            HttpResponse::BadRequest().body("Transformation failed")
        }
    } 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(transform)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}