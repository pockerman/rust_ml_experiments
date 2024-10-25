use actix_web::{App, web, HttpServer, middleware::Logger};
mod configuration;
use configuration::server_config::get_server_config;
use mongodb::{Client};
mod views;


// Entry point for the application
#[actix_web::main]
async fn main() -> std::io::Result<()>{
	
	// set the logs level
	std::env::set_var("RUST_LOG", "debug");
	
	env_logger::init_from_env(env_logger::Env::new()
                              .default_filter_or("info"));
	
	// get the configuration for the server
    
	let config = get_server_config();
	
	// get the mongodb ur and connect
	let mdb_uri = config.mongodb_uri;
	
	// Create a new client and connect to the server
    let client = Client::with_uri_str(mdb_uri).await.expect("failed to connect");
	
	
	
	let app_url = config.host + ":" + &config.port.to_string();
	
	println!("Starting application on {}", app_url);
	
	let config = get_server_config();
	let server_config = (config.host, config.port);
    HttpServer::new(move || {
        let app = App::new()
		.app_data(web::Data::new(client.clone()))
		.wrap(Logger::new("%a %{User-Agent}i"))
		.configure(views::views_factory);
        return app
    })
    .bind(server_config)?
    .run()
    .await
}