#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::request::Form;
use chrono::NaiveDate;
use validator::{validate, Validate};
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct UserLogin {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserRegistration {
    name: String,
    #[validate(custom = "validate_password")]
    password: String,
    #[validate(email)]
    email: String,
    date_of_birth: NaiveDate,
    gender_description: Option<String>,
}

fn validate_password(input: &String) -> Result<(), validator::ValidationFieldError> {
    let re = Regex::new(r"^[0-9a-fA-F]{40}$").unwrap();
    if re.is_match(input) {
        Ok(())
    } else {
        Err(validator::ValidationFieldError::with_message("密码不符合规则", "password"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct MovieRecord {
    title: String,
    director: String,
    starring: Vec<String>,
    details: Option<String>,
    staffs: Option<Vec<String>>,
    rental_price: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct MovieRentalRecord {
    user_id: i32,
    movie_id: i32,
    rental_date: NaiveDate
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search/<name>")]
fn search(name: &str) -> &'static str {
    "Hello, world!"
}

#[get("/browse/<id>")]
fn browse(id: &int) -> &'static str {
    "Hello, world!"
}

#[post("/wish-item/<movie_id>")]
fn wish_item(movie_id: &int) -> String {
    format!("Hello, {}!", login.name)
}

#[post("/rent-item", data = "<item>")]
fn rent_item(item: Form<MovieRentalRecord>) -> String {
    format!("Hello, {}!", item.name)
}

#[post("/add-item", data = "<item>")]
fn add_item(item: Form<MovieRecord>) -> String {
    format!("Hello, {}!", item.name)
}

#[post("/register", data = "<data>")]
fn register(data: Form<UserRegistration>) -> String {
    format!("Hello, {}!", data.name)
}

#[post("/login", data = "<login>")]
fn login(login: Form<UserLogin>) -> String {
    format!("Hello, {}!", login.name)
}

#[get("/home")]
async fn serve_index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("htdocs/index.html").await
}

#[get("/")]
async fn serve_login() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("htdocs/login.html").await
}

#[get("/search")]
async fn serve_search() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("htdocs/search.html").await
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, search, browse,
                                          add_item, rent_item, wish_item])
    rocket::build().mount("/", routes![serve_index, serve_login, serve_search])
}
