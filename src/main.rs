use qdrant_client::prelude::*;
use qdrant_client::qdrant::{CreateCollection, VectorParams};
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::VectorsConfig;
use qdrant_client::prelude::Distance;
use rand::Rng;
use std::collections::HashMap;
use serde_json::json;
use anyhow::{anyhow, Result};

#[derive(Debug)]
struct VectorStatistics {
    min: f32,
    mean: f32,
    median: f32,
    max: f32,
}

fn generate_random_vector(dim: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dim).map(|_| rng.gen()).collect()
}

fn calculate_statistics(vector: &Vec<f32>) -> VectorStatistics {
    let mut sorted_vector = vector.clone();
    sorted_vector.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    VectorStatistics {
        min: *sorted_vector.first().unwrap(),
        mean: vector.iter().sum::<f32>() / vector.len() as f32,
        median: sorted_vector[sorted_vector.len() / 2],
        max: *sorted_vector.last().unwrap(),
    }
}

async fn insert_vector_data(client: &QdrantClient, collection_name: &str, vectors: &HashMap<u64, VectorStatistics>) -> Result<()> {
    for (index, stats) in vectors.iter() {
        let payload = json!({
            "min": stats.min,
            "mean": stats.mean, 
            "median": stats.median,
            "max": stats.max
        });

        // Adjusted error handling here
        let payload: Payload = payload.try_into().map_err(|e| anyhow!("Payload conversion error: {:?}", e))?;

        let vector = generate_random_vector(10); // Make sure this matches the dimensionality of your vectors
        client.upsert_points_blocking(collection_name, None, vec![PointStruct::new(*index, vector, payload)], None).await?;
    }

    Ok(())
}

async fn search_and_display_results(client: &QdrantClient, collection_name: &str) -> Result<()> {
    let query_vector: Vec<f32> = vec![0.5; 10]; // Adjust as needed

    let search_result = client.search_points(&SearchPoints {
        collection_name: collection_name.into(),
        vector: query_vector,
        filter: None,
        limit: 3,
        with_payload: Some(true.into()),
        ..Default::default()
    }).await?;

    println!("Search Results:");
    for (index, point) in search_result.result.iter().enumerate() {
        // Adjusted formatting here
        println!("Point {}: ID = {:?}, Payload = min:{:?}, mean: {} ", index, point.id, point.payload, point.payload["mean"]);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config))?;
    let collection_name = "VectorCollection";
    let dimension = 10;
    let create_collection_result = client.create_collection(&CreateCollection {
        collection_name: collection_name.to_string(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: dimension, // Specify the vector size
                distance: Distance::Cosine.into(), // Convert `Distance` to `i32`
                ..Default::default()
            })),
            ..Default::default()
        }),
        ..Default::default()
    }).await;

    match create_collection_result {
        Ok(_) => println!("Collection '{}' was created.", collection_name),
        Err(e) => println!("Error creating collection '{}': {:?}", collection_name, e),
    }

    let num_vectors = 3;
    let dim = 10;
    let mut stats = HashMap::new();

    for i in 0u64..num_vectors {
        let vector: Vec<f32> = generate_random_vector(dim);
        let stat = calculate_statistics(&vector);
        stats.insert(i, stat);
    }

    insert_vector_data(&client, collection_name, &stats).await?;
    search_and_display_results(&client, collection_name).await?;

    Ok(())
}


