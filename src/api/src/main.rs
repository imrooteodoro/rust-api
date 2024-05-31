#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {name:"World"})
}

#[get("/<x>/<y>")]
fn sum(x: u32, y: u32) -> Template {
    Template::render("sum", context! {soma:(x + y).to_string()})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/sum", routes![sum])
        .attach(Template::fairing())
       
}
