#![feature(plugin)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::{tera::Context, Template};
//use tera::Context;

//use rocket;

fn main() {
    rocket::ignite().   //creates a rocket instance
    mount("/", routes![index])  //the first arg is the path, the second is the function
    .attach(Template::fairing())  //attach the tera template to render the index page
    .launch(); //launch the server
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("my_message", "hello world");
    Template::render("layout", &context)
}
