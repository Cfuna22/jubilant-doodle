use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id:Uuid,
    pub title: String,
}