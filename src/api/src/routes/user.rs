use rocket::serde::Deserialize;


#[derive(Debug, PartialEq, Eq, Deserialize)]
struct User{
    id: i64, 
    usr_name:String,
    usr_email: String,
    usr_active: bool,
}
#[post("/user")]
pub fn user() -> String {
    format!("is Working")
}