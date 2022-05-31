use std::io::Write;
use actix_web::{web, get, post, HttpResponse, Error, http::header::ContentType};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/image")
		.service(show)
		.service(upload)
	);
}

#[get("/{md5}")]
pub async fn show(md5: web::Path<String>) -> HttpResponse {
	println!("md5{}", md5);
	
	HttpResponse::Ok()
	.content_type(ContentType::plaintext())
	.body("OK")
}

#[post("/upload")]
pub async fn upload(mut image: Multipart) -> Result<HttpResponse, Error> {
	println!("开始上传图片");
	while let Some(mut field) = image.try_next().await? {
		let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| uuid::Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./tmp/{}", filename);
        
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;
		
		while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
	}

	Ok(HttpResponse::Ok().content_type(ContentType::plaintext()).body("上传成功"))
}
