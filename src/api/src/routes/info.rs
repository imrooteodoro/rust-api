use rocket_dyn_templates::{Template, context};

#[get("/<name>/<age>")]
fn info(name: &str, age: u8) -> Template {
    Template::render("info", context! { nome: name, idade: age })
}

