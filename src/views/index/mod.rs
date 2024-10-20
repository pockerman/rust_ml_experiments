pub mod index_routes;

use actix_web::web::{ServiceConfig};

pub fn index_views_factory(app: &mut ServiceConfig){

	// init the routes for data management
	index_routes::init(app);

}