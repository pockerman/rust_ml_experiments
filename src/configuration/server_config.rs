pub struct ServerConfig{
	pub port: u16,
	pub host: String,
	pub db_name: String,
	pub mongodb_uri: String
}

pub fn get_server_config() -> ServerConfig{
	let config = ServerConfig{
		port:8080,
		host: String::from("127.0.0.1"),
		db_name: String::from("rust-ml-experiment"),
		//MONGODB_URI: String::from("mongodb://localhost:27017")
		mongodb_uri: String::from("mongodb+srv://AlexRustML:da13div08pao@mir-app-cluster-0.cs5ihg4.mongodb.net/?retryWrites=true&w=majority&appName=mir-app-cluster-0")
	};
	config	
}