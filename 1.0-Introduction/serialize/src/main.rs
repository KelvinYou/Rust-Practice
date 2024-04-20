use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    state: Status,
}

#[derive(Serialize, Deserialize)]
enum Status {
    Stop,
    Running,
    Skipped,
}

fn main() {
    let task = Task {
        id: 1,
        state: Status::Running,
    };

    // Serialize the struct to JSON
    let json_output = serde_json::to_string_pretty(&task).unwrap();
    println!("{}", json_output);
}
