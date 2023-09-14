use actix_files::*;
use std::path::PathBuf;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::*;

/// https://actix.rs/docs/static-files/
async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./frontend/dist/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/{filename:.*}")]
async fn files(req: HttpRequest) -> Result<NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index))
        .service(
            Files::new("/", "./frontend/dist")
                .show_files_listing()
                .use_last_modified(true),
        ))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
