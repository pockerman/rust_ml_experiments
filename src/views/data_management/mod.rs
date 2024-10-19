pub mod dm_routes;


use actix_web::web::{ServiceConfig};

pub fn dm_views_factory(app: &mut ServiceConfig){

	dm_routes::init(app);
//    app.service(scope("v1/dm/upload-ds-file").route("upload-ds-file",
//                get().to(dm_routes::upload_dataset_file))
//    );

}