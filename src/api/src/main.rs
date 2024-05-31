#[macro_use]
extern crate rocket;

mod routes {
    pub mod index;
    pub mod info;
    pub mod sum;
    pub mod user;
}

use rocket_dyn_templates::{Template};
use routes::index::*;
use routes::sum::*;
use routes::info::*;
use routes::user::*;





#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/sum", routes![sum])
        .mount("/info", routes![info])
        .mount("/", routes![user])
        .attach(Template::fairing())
        
}
