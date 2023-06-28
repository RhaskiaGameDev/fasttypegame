use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn process_data(data: web::Json<String>) -> impl Responder {
    // Process the received data here
    let result = data.to_uppercase();

    HttpResponse::Ok().body(result)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/process")
                .route(web::post().to(process_data)))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}