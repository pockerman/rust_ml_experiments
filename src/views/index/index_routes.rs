use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// Upload a file defining a dataset to the user
/// specified space
//#[get("/index")]
async fn get_index(info: web::Path<Info>) -> Result<HttpResponse, actix_web::error::Error> {
	//format!("Welcome {}!", info.username)
	Ok(HttpResponse::Ok().body(format!("Welcome {}!", info.username)))
}

pub fn init(cfg: &mut web::ServiceConfig){
	
	
	cfg.service(web::resource("/v1/index/{username}")
	           .route(web::get().to(get_index)));
        
}


