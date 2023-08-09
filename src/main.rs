#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("your id: {}, is a unsigned integer", id)
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("your id: {}, is a signed integer", id)
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String {
    format!("your id: {}, is a string", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, user, user_int, user_str])
}
