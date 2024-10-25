use futures::{StreamExt};
use actix_multipart::Multipart;
use actix_files;
use actix_web::{web, post, get, HttpResponse};
use tokio::io::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use crate::configuration;
//use configuration::server_config::get_server_config;



#[derive(serde::Deserialize)]
struct TemplateInfo {
    name: Option<String>,
}


#[derive(serde::Deserialize)]
struct DatasetSearchQuery {
    id: Option<String>,
}




#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}




/*pub async fn serve_directory_listing(path: web::Path<String>) -> HttpResponse {
    let directory = format!("./{}", path);
    match tokio::fs::read_dir(&directory).await {
        Ok(mut entries) => {
            let mut html = format!("<html><body><h1>Directory Listing: {}</h1><ul>", path);
            while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let file_url = format!("{}/{}", path, file_name);
                html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", file_url, file_name));
            }
            html.push_str("</ul></body></html>");
            HttpResponse::Ok().content_type("text/html").body(html)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}*/



pub fn init(cfg: &mut web::ServiceConfig){
	
	
	/*cfg
        .service(upload_dataset_file)
        .service(web::scope("/dm/")
                .service(actix_files::Files::new("/", "./assets").show_files_listing())
                .default_service(web::route().to(serve_directory_listing)),
        );
        /*.service(web::scope("/templates")
                .service(actix_files::Files::new("/", "./templates").show_files_listing())
                .default_service(web::route().to(serve_directory_listing)),
        );*/
		*/
	cfg.service(get_dataset_info);

	
}

/*pub async fn save_file(mut payload: Multipart, 
                       file_path: std::path::PathBuf) -> Result<HttpResponse, actix_web::error::Error> {
						   
		let mut file = tokio::fs::File::create(file_path).await?;
		
		while let Some(field) = payload.next().await{
			
			let mut field = match field {
				Ok(field) => field,
				Err(e)  => return Err(actix_web::error::ErrorBadRequest(e.to_string())),
			};
			
			if field.name() == Some("file") {
				
				while let Some(chunk) = field.next().await{
					
					let chunk = match chunk {
						Ok(chunk) => chunk,
						Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string()))
					};

					let _ = file.write_all(&chunk).await?;
					
				}
			}
			
		}
		
		Ok(HttpResponse::Ok().body("File saved successfully"))
    }
*/

/// Upload a file defining a dataset to the user
/// specified space
/*#[post("/upload-ds-file")]
pub async fn upload_dataset_file(payload: Multipart,
                                info: web::Query<TemplateInfo>) -> Result<HttpResponse, actix_web::error::Error> {
  
									
	let file_path = std::path::PathBuf::from(format!("./templates/{}", 
	                                         info.name.clone().unwrap_or_else(|| "unnamed".to_string())));
    let upload_status = save_file(payload, 
	                              file_path).await;

	upload_status
}*/

/// Get the dataset information
#[get("/dm/datasets/{id}/info")]
pub async fn get_dataset_info(query: web::Query<DatasetSearchQuery>,
                              conn: web::Data<Client>) -> Result<HttpResponse, actix_web::error::Error>{
								  
								  
	let config = configuration::server_config::get_server_config();
	// Get a handle on the movies collection
    //let database = client.database("rust-ml-experiment");
	let db_name: String = config.db_name;
    const COLL_NAME: &str = "users";
	
    //let my_coll: Collection<Document> = database.collection("datasets");
	
	let collection: Collection<User> = conn.database(&db_name).collection(COLL_NAME);
	
    // Find a movie based on the title value
    //let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;
								  
	Ok(HttpResponse::Ok().body("Here is the dataset"))
	
	
}
