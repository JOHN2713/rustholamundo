#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hola Mundo desde Rust!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
