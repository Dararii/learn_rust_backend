use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[get("/accounts")]
async fn get_accounts() -> impl Responder {
    HttpResponse::Ok().body("All Accounts are gotten")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(get_accounts)).bind("127.0.0.1:8080")?.run().await
}
