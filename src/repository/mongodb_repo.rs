use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{ DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection}, Cursor,

};

use crate::models::user_model::Receta;

pub struct MongoRepo{
    col: Collection<Receta>,
} 

impl MongoRepo{
    pub fn init() -> Self {
        dotenv().ok();
        let uri= match env::var("MONGOURI"){
            Ok(v)=> v.to_string(),
            Err(_)=> format!("Error loading env variable")
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db= client.database("projectFood");
        let col: Collection<Receta> = db.collection("recipes");
        MongoRepo{ col }
    }

    pub fn create_receta(&self, new_receta:Receta) -> Result<InsertOneResult, Error>{
        let new_doc = Receta{
            id:None,
            categoria: new_receta.categoria,
            descripcion: new_receta.descripcion,
            precio: new_receta.precio,
            ingredientes: new_receta.ingredientes
        };
        let receta= self
            .col
            .insert_one(new_doc,None)
            .ok()
            .expect("Error al  crear la receta");
        Ok(receta)
    }


pub fn get_receta(&self, id: &String) -> Result<Receta, Error>{
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let receta_detail= self
        .col
        .find_one(filter,None)
        .ok()
        .expect("Error gettin recetas detail");
    Ok(receta_detail.unwrap())
}

pub fn update_receta(&self, id: &String, new_receta: Receta) -> Result<UpdateResult, Error>{
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let new_doc = doc! {
        "$set":
        {
            "id": new_receta.id,
            "categoria": new_receta.categoria,
            "precio": new_receta.precio,
            "descripcion": new_receta.descripcion,
            "ingredientes": new_receta.ingredientes
        },
    };
    let updated_doc = self 
        .col
        .update_one(filter, new_doc, None)
        .ok()
        .expect("Error updating recetas");
    Ok(updated_doc)
}

pub fn delete_receta(&self, id: &String)-> Result<DeleteResult, Error>{
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let recetas_detail = self
        .col
        .delete_one(filter, None)
        .ok()
        .expect("Error detaling recetas");
    Ok(recetas_detail)
}

pub fn get_all_recetas(&self)-> Result<Vec<Receta>, Error>{
    let cursors = self
        .col
        .find(None, None)
        .ok()
        .expect("Error getting list of recetas");
    let recetas = cursors.map(|doc| doc.unwrap()).collect();
    Ok(recetas)
}
}