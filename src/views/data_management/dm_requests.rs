use serde::{Deserialize, Serialize};

/// Create a new dataset
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewDatasetRequest{
	
	pub name: String,
	pub version: String,
	pub n_examples: u32,
	pub genre: String,
	pub description: String,
}