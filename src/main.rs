

use std::fs;

use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};




#[get("/")]
async fn hello() -> impl Responder {


    let html_content = match fs::read_to_string("templates/index.html") {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Error reading the HTML file"),
    };

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html_content)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
}


