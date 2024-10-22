use actix_web::{App, HttpServer, middleware::Logger};
mod configuration;
use configuration::server_config::get_server_config;
mod views;


// Entry point for the application
#[actix_web::main]
async fn main() -> std::io::Result<()>{
	
	// get the configuration for the server
    
	let config = get_server_config();
	
	let app_url = config.host + ":" + &config.port.to_string();
	
	println!("Starting application on {}", app_url);
	
	let config = get_server_config();
	let server_config = (config.host, config.port);
    HttpServer::new(|| {
        let app = App::new()
		.wrap(Logger::new("%a %{User-Agent}i"))
		.configure(views::views_factory);
        return app
    })
    .bind(server_config)?
    .run()
    .await
}