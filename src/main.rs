#[macro_use] extern crate rocket;

mod config;

mod faces;
use faces::Faces;

use rocket::data::Data;
use rocket_contrib::json::Json;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FaceResponse {
    engagement_score: usize
}

#[post("/engagement/score", format = "image/*", data = "<image>")]
async fn engagement_score(image: Data) -> Json<FaceResponse> {
    let faces = Faces::new(config::KEY, config::ENDPOINT, image).await.0;

    let mut largest_face = None;
    let mut largest_face_area: u32 = 0;
    for face in faces.iter() {
        let face_area = face.face_rectangle.get_area();
        if face_area > largest_face_area {
            largest_face = Some(face);
            largest_face_area = face_area;
        } 
    }
    
    match largest_face {
        Some(face) => Json(FaceResponse { engagement_score: face.engagement_score() }),
        None => Json(FaceResponse { engagement_score: 0 })
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::ignite()
        .mount("/", routes![index, engagement_score])
        .launch().await.unwrap();
}