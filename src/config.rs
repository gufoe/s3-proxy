#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub workers: Option<usize>,
    pub host: String,
    pub port: u16,

    // S3 options
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_host: String,
    pub s3_key: String,
    pub s3_secret: String,
}
pub fn read_config(path: &std::path::PathBuf) -> Config {
    use std::fs::File;
    use std::io::Read;

    let mut content = String::new();
    let mut file = File::open(path).expect("Cannot open configuration file");
    file.read_to_string(&mut content)
        .expect("Cannot open configuration file");

    toml::from_str(&content).unwrap()
}
