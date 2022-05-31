use actix_web::*;
use image_cloud::api::image_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(||{
		App::new().configure(image_api::config)
	}).bind(("localhost", 8080))?
	.run()
	.await
}

