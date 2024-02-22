mod api;
mod models;
mod repository;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3000"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[macro_use]
extern crate rocket;

use api::user_api::{
    create_receta, 
    get_receta, 
    update_receta, 
    delete_receta,
    get_all_recetas
};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket()-> _{
    let db= MongoRepo::init();
    rocket::build()
    .manage(db)
    .mount("/", routes![create_receta])
    .mount("/", routes![get_receta])
    .mount("/", routes![update_receta])
    .mount("/", routes![delete_receta])
    .mount("/", routes![get_all_recetas])
    .attach(CORS)
}