use actix_web::{get, post, HttpResponse, HttpServer, Responder, web, App};

#[get("/")]
async fn get_all_items() -> impl Responder {
    //placeholder: returns hello world
    HttpResponse::Ok().body("Hello world")
}

#[post("/")]
async fn add_item(req_body: String) -> impl Responder {
    //placeholder: echoes list item name
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_items)
            .service(add_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
