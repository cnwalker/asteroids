use std::error::Error;
use tokio::runtime::Runtime;
mod asteroids;

async fn get_asteroid_close_approaches() -> serde_json::Value {
    let params = [
        ("date-min", "2025-01-01"),
        ("date-max", "2030-01-01"),
        ("fullname", "true"),        
        // Must be <= 0.2 astronomical units at closest pass
        ("dist-max", "0.2"),
        // Near earth asteroid
        ("nea", "true"),
    ];
    let url =
        reqwest::Url::parse_with_params("https://ssd-api.jpl.nasa.gov/cad.api", &params).unwrap();
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();
    let asteroid_approaches = serde_json::from_str::<serde_json::Value>(&body).unwrap();
    return asteroid_approaches;
}

fn extract_desirable_asteroids(filepath: &str) -> Vec<asteroids::Asteroid> {
    let doc_reader = csv::Reader::from_path(filepath);
    let re = regex::Regex::new(r"\(.*\)").unwrap();
    let mut all_asteroids = Vec::new();
    for result in doc_reader?.deserialize() {
        let mut asteroid: asteroids::Asteroid = result.unwrap();
        let trimmed_name = asteroid.full_name.trim();
        let name = re.find(&trimmed_name).unwrap();
        asteroid.full_name = name.as_str().to_string();
        all_asteroids.push(asteroid);
    }
    return all_asteroids;
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let close_approach_asteroids = runtime.block_on(get_asteroid_close_approaches());
    let desirable_asteroids = extract_desirable_asteroids("close_potentially_metallic_asteroids.csv");
    println!("{:?}", desirable_asteroids);
}
