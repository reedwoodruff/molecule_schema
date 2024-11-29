use std::{fs::File, io::BufReader};

use base_types::post_generation::StandaloneRGSOConcrete;

mod save_graph;

#[tokio::main]
async fn main() {
    let file_path = "./json_to_pipe.json";

    match read_json_from_file(file_path) {
        Ok(data) => {
            save_graph::save_graph(data).await;
        }
        Err(e) => println!("Error reading JSON: {}", e),
    }
}

fn read_json_from_file(file_path: &str) -> serde_json::Result<Vec<StandaloneRGSOConcrete>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(file_path);
    let file = file.unwrap_or_else(|_| panic!("file not found"));
    let reader = BufReader::new(file);

    // Parse the JSON data from the file.
    let data: Vec<StandaloneRGSOConcrete> = serde_json::from_reader(reader)?;

    Ok(data)
}
