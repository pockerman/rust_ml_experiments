pub struct ServerConfig{
	pub const PORT: u16,
	pub const HOST: String,
	pub const DB_NAME: String,
	pub const MONGODB_URI: String
}

pub fn get_server_config() -> ServerConfig{
	let config = ServerConfig{
		PORT:8080,
		HOST: String::from("127.0.0.1"),
		DB_NAME: String::from("rust-ml-experiment"),
		MONGODB_URI: String::from("mongodb://localhost:27017")
	};
	config	
}