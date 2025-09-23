use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub addr: String,
    pub db_path: String,
    pub index_dir: Vec<String>
}

impl Config {
    pub async fn load(path: &str) -> Self {
        let file = match tokio::fs::read_to_string(path).await {
            Ok(f) => f,
            Err(_) => panic!("")
        };

        serde_json::from_str(file.as_str()).unwrap()
    }
}