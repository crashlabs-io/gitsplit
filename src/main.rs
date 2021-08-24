#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/payload", data = "<input>")]
fn payload(input: String) -> String {
    println!("{:#?}", input.clone());
    input
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![payload])
}
