use rocket_dyn_templates::{Template, context};

#[get("/<x>/<y>")]
pub fn sum(x: u32, y: u32) -> Template {
    Template::render("sum", context! { soma: (x + y).to_string() })
}