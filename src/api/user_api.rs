
use crate::{models::user_model::Receta, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json,State};

#[post("/receta", data="<new_receta>")]
pub fn create_receta(
    db: &State<MongoRepo>,
    new_receta: Json<Receta>,
) -> Result <Json<InsertOneResult>, Status>{
    let data= Receta{
        id:None,
        categoria: new_receta.categoria.to_owned(),
        descripcion: new_receta.descripcion.to_owned(),
        precio: new_receta.precio.to_owned(),
        ingredientes: new_receta.ingredientes.to_owned(),
    };
    let receta_detail = db.create_receta(data);
    match receta_detail{
        Ok(receta) => Ok(Json(receta)),
        Err(_)=> Err(Status::InternalServerError)
    }
}

#[get("/receta/<path>")]
pub fn get_receta(db: &State<MongoRepo>, path:String) -> Result<Json<Receta>, Status>{
    let id= path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    }
    let receta_detail = db.get_receta(&id);
    match receta_detail{
        Ok(receta) => Ok(Json(receta)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[put("/recta/<path>", data="<new_receta>")]
pub fn update_receta(
    db: &State<MongoRepo>,
    path: String,
    new_receta: Json<Receta>,
) -> Result<Json<Receta>, Status>{
    let id= path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let data= Receta{
        id: Some(ObjectId::parse_str(&id).unwrap()),
        categoria: new_receta.categoria.to_owned(),
        descripcion: new_receta.descripcion.to_owned(),
        precio: new_receta.precio.to_owned(),
        ingredientes: new_receta.ingredientes.to_owned(),
    };

    let update_result = db.update_receta(&id, data);
    match update_result{
        Ok(update) =>{
            if update.matched_count==1{
                let updated_receta_info = db.get_receta(&id);
                match updated_receta_info{
                    Ok(receta) => Ok(Json(receta)),
                    Err(_)=> Err(Status::InternalServerError),
                }
            }else{
                Err(Status::NotFound)
            }
        }
        Err(_) =>Err(Status::InternalServerError),
    }

}

#[delete("/recetas/<path>")]
pub fn delete_receta(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status>{
        let id = path;
        if id.is_empty(){
            return Err(Status::BadRequest);
        };
        let result = db.delete_receta(&id);
        match result {
            Ok(res)=>{
               if res.deleted_count==1{
                Ok(Json("User successfully deleted!"))
               } else{
                Err(Status::NotFound)
               }
            }
            Err(_)=>Err(Status::InternalServerError)
        }
    }

#[get("/recetas")]
pub fn get_all_recetas(db: &State<MongoRepo>)-> Result<Json<Vec<Receta>>, Status>{
    let recetas = db.get_all_recetas();
    match recetas {
        Ok(recetas)=>Ok(Json(recetas)),
        Err(_) => Err(Status::InternalServerError),
    }
}


