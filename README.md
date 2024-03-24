# Mini-project 7: Vector Database in Rust

This Rust program creates a vector database using the Qdrant client. It generates random vectors, calculates their statistics (min, mean, median, max), and stores them in a Qdrant collection. It also provides a function to search for vectors in the collection and display their statistics.

## Prject Structure
The main functionality is contained in `main.rs`. Here's a brief overview of the functions:

- `generate_random_vector(dim: usize) -> Vec<f32>`: Generates a random vector of the specified dimension.
- `calculate_statistics(vector: &Vec<f32>) -> VectorStatistics`: Calculates the min, mean, median, and max of a vector.
- `insert_vector_data(client: &QdrantClient, collection_name: &str, vectors: &HashMap<u64, VectorStatistics>) -> Result<()>`: - Inserts the vectors and their statistics into a Qdrant collection.
- `search_and_display_results(client: &QdrantClient, collection_name: &str) -> Result<()>`: Searches for vectors in the collection and displays their statistics.
## Code Overview

The code consists of several functions and a main function that runs the program.

### Struct: VectorStatistics

```rust
#[derive(Debug)]
struct VectorStatistics {
    min: f32,
    mean: f32,
    median: f32,
    max: f32,
}
```

This struct is used to store the statistics of a vector.

**Function: generate_random_vector**

```rust
fn generate_random_vector(dim: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dim).map(|_| rng.gen()).collect()
}
```
This function generates a random vector of the specified dimension.

**Function: calculate_statistics**
```rust
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
```

This function calculates the min, mean, median, and max of a vector.

**Function: insert_vector_data**

```rust
async fn insert_vector_data(client: &QdrantClient, collection_name: &str, vectors: &HashMap<u64, VectorStatistics>) -> Result<()> {
    for (index, stats) in vectors.iter() {
        let payload = json!({
            "min": stats.min,
            "mean": stats.mean, 
            "median": stats.median,
            "max": stats.max
        });

        let payload: Payload = payload.try_into().map_err(|e| anyhow!("Payload conversion error: {:?}", e))?;

        let vector = generate_random_vector(10);
        client.upsert_points_blocking(collection_name, None, vec![PointStruct::new(*index, vector, payload)], None).await?;
    }

    Ok(())
}
```

This function inserts the vectors and their statistics into a Qdrant collection.

**Function: search_and_display_results**
```rust
async fn search_and_display_results(client: &QdrantClient, collection_name: &str) -> Result<()> {
    let query_vector: Vec<f32> = vec![0.5; 10];

    let search_result = client.search_points(&SearchPoints {
        collection_name: collection_name.into(),
        vector: query_vector,
        filter: None,
    }
}
```

## Running guidelines:

Before running the project, make sure you have a running instance of Qdrant. You can start a Qdrant instance using Docker (open Docker software):
```docker
docker run -p 6333:6333 -p 6334:6334 -e QDRANT__SERVICE__GRPC__ENABLED=true qdrant/qdrant
```

Ensure that you have installed Rust, Cargo. Use the following command to run the project:
```
cargo run
```
## Dependencies
This project uses the following dependencies:

`qdrant-client`: A Rust client for Qdrant.
`rand`: A Rust library for generating random numbers.
`serde_json`: A Rust library for serializing and deserializing JSON.
`tokio`: A Rust runtime for asynchronous programming.

## Screenshots

- Instance of Qdrant:
![Qdrant Instance](/images/docker.png)

- Query result visualization:
![result](/images/terminalResult.png)

