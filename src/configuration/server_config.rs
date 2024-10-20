pub struct ServerConfig{
	pub port: u16,
	pub host: String,
}

pub fn get_server_config() -> ServerConfig{
	let config = ServerConfig{
		port:8080,
		host: String::from("127.0.0.1")
	};
	config	
}