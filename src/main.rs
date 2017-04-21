#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
use std::io;

use rocket::request::Form;

#[derive(FromForm)]
struct EditForm {
    content: String
}

#[derive(FromForm)]
struct EditForm2<'r> {
    content: &'r str
}

#[post("/form1", data="<data>")]
fn form1(data: Form<EditForm>) -> io::Result<String> {
    Ok(format!("msg: {}\n", data.into_inner().content))
}

#[post("/form2", data="<data>")]
fn form2<'r>(data: Form<'r, EditForm2<'r>>) -> io::Result<String> {
    Ok(format!("msg: {}\n", data.get().content))
}

fn main() {
    rocket::ignite().mount("/", routes![form1, form2]).launch();
}