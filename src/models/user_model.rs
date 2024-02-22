use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Receta {
    #[serde(rename = "_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub categoria: String,
    pub precio: f64,
    pub descripcion: String,
    pub ingredientes: Vec<String>,
}

