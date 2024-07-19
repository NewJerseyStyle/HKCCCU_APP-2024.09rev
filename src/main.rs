#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::post;
use rocket::request::Form;
use chrono::NaiveDate;
use validator::{validate, Validate};
use regex::Regex;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

mod diesel;
mod schema;

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
fn search(name: &str, conn: diesel::mysql::MysqlConnection) -> String {
    use schema::movies::dsl::*;
    let results = movies.filter(title.like(format!("%{}%", name))).load::<schema::Movie>(&conn).unwrap();
    let mut response = "Search results:\n".to_string();
    for movie in results {
        response.push_str(&format!("{} - {}\n", movie.title, movie.director));
    }
    response
}

#[get("/browse/<id>")]
fn browse(id: i32, conn: diesel::mysql::MysqlConnection) -> String {
    use schema::movies::dsl::*;
    let movie = movies.find(id).first::<schema::Movie>(&conn).unwrap();
    format!("Movie {} - {}", movie.title, movie.director)
}

#[post("/wish-item/<movie_id>")]
fn wish_item(movie_id: i32, conn: diesel::mysql::MysqlConnection, login: Form<UserLogin>) -> String {
    use schema::user_wishes::dsl::*;
    let wish = schema::UserWish {
        user_id: 1, // placeholder, should be the actual user ID
        movie_id,
    };
    diesel::insert_into(user_wishes)
        .values(&wish)
        .execute(&conn)
        .unwrap();
    format!("Wish added for {}", login.name)
}

#[post("/rent-item", data = "<item>")]
fn rent_item(item: Form<MovieRentalRecord>, conn: diesel::mysql::MysqlConnection) -> String {
    diesel::insert_into(schema::movie_rentals::table)
        .values(&item.into_inner())
        .execute(&conn)
        .unwrap();
    format!("Rental added for {}", item.user_id)
}

#[post("/add-item", data = "<item>")]
fn add_item(item: Form<MovieRecord>, conn: diesel::mysql::MysqlConnection) -> String {
    diesel::insert_into(schema::movies::table)
        .values(&item.into_inner())
        .execute(&conn)
        .unwrap();
    format!("Movie added: {}", item.title)
}

#[post("/register", data = "<data>")]
fn register(data: Form<UserRegistration>, conn: diesel::mysql::MysqlConnection) -> String {
    diesel::insert_into(schema::users::table)
        .values(&data.into_inner())
        .execute(&conn)
        .unwrap();
    format!("User registered: {}", data.name)
}

#[post("/login", data = "<login>")]
fn login(login: Form<UserLogin>, conn: diesel::mysql::MysqlConnection) -> String {
    use schema::users::dsl::*;
    let user = users.filter(name.eq(&login.name)).first::<schema::User>(&conn).unwrap();
    if user.password == login.password {
        format!("Welcome, {}!", login.name)
    } else {
        "Invalid password".to_string()
    }
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
    let conn = diesel::establish_connection();
    rocket::build().mount("/api", routes![index, search, browse,
                                          add_item, rent_item, wish_item,
                                          login, register])
    rocket::build().mount("/", routes![serve_index, serve_login, serve_search])
}
