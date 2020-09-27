#[macro_use] extern crate rocket;

mod config;

mod faces;
use faces::Faces;

use rocket::data::Data;
use rocket_contrib::json::Json;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FaceResponse {
    engagement_score: usize
}

#[post("/engagement/score", format = "image/*", data = "<image>")]
async fn engagement_score(image: Data) -> Json<FaceResponse> {
    // Find the largest face in the image
    let faces = Faces::new(config::KEY, config::ENDPOINT, image).await.0;
    let largest_face = faces.iter().max_by_key(|face|
        face.face_rectangle.get_area());
    
    // Calculate and return the engagement score
    match largest_face {
        Some(face) => Json(FaceResponse { engagement_score: face.engagement_score() }),
        None => Json(FaceResponse { engagement_score: 0 })
    }
}

#[rocket::main]
async fn main() {
    rocket::ignite()
        .mount("/", routes![engagement_score])
        .launch().await.unwrap();
}