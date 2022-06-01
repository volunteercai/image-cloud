use actix_web::{web, get, post, HttpResponse};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/user")
		.service(register).service(login)
		.service(current_info).service(update)
	);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	name: Option<String>,
	email: Option<String>,
	password: Option<String>,
}

#[post("/register")]
pub async fn register() -> HttpResponse {
	HttpResponse::Ok().body("body")
}

#[post("/login")]
pub async fn login() -> HttpResponse {
	HttpResponse::Ok().body("body")
}

#[get("/current-info")]
pub async fn current_info() -> HttpResponse {
	HttpResponse::Ok().json(User{
		name: Option::Some("".to_string()),
		email: Option::Some("".to_string()),
		password: Option::Some("".to_string())
	})
}

#[post("/update")]
pub async fn update() -> HttpResponse {
	HttpResponse::Ok().body("body")
}


