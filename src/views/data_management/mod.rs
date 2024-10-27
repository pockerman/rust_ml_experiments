pub mod dm_routes;
pub mod dm_requests;	

use actix_web::web::{ServiceConfig};

pub fn dm_views_factory(app: &mut ServiceConfig){

	// init the routes for data management
	dm_routes::init(app);

}