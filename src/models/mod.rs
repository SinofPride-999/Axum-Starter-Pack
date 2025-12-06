use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExampleModel {
    pub id: u32,
    pub name: String,
}
