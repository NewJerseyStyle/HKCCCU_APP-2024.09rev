#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::get;

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

#[post("/add-item", data = "<item>")]
fn add_item(login: Form<MovieRecord>) -> String {
    format!("Hello, {}!", login.name)
}

#[post("/register", data = "<login>")]
fn login(login: Form<UserLogin>) -> String {
    format!("Hello, {}!", login.name)
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

#[get("/upload")]
async fn serve_search() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("htdocs/add-item.html").await
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index,])
    rocket::build().mount("/", routes![serve_index, serve_login, serve_search])
}