pub mod data_management;
use data_management::dm_views_factory;


use actix_web::web::ServiceConfig;

/// Views factory
pub fn views_factory(app: &mut ServiceConfig) {
    dm_views_factory(app);
}