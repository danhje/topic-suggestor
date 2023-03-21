#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/fix?<msg>")]
fn fix(msg: String) -> String {
    format!("Hello, {msg}!")
}

fn main() {
    rocket::ignite().mount("/", routes![index, fix]).launch();
}
