#[macro_use] extern crate rocket;

mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch().await.unwrap();
}