#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;
use duckdb::{Connection, Result};
use std::sync::Mutex;

struct AppState {
    conn: Mutex<Connection>,
}

fn initialize_db(conn: &Connection) {
    let sql = r#"
    -- Users table
    CREATE TABLE IF NOT EXISTS Users (
        UserID INTEGER PRIMARY KEY AUTOINCREMENT,
        Username VARCHAR(50) NOT NULL UNIQUE,
        PasswordHash VARCHAR(255) NOT NULL,
        Email VARCHAR(100) NOT NULL UNIQUE,
        DateOfBirth DATE NOT NULL,
        GenderDescription VARCHAR(50),
        CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    -- Movies table
    CREATE TABLE IF NOT EXISTS Movies (
        MovieID INTEGER PRIMARY KEY AUTOINCREMENT,
        Title VARCHAR(255) NOT NULL,
        Director VARCHAR(255) NOT NULL,
        Starring TEXT NOT NULL,
        Details TEXT,
        Staffs TEXT,
        RentalPrice DECIMAL(5, 2)
    );

    -- Genres table
    CREATE TABLE IF NOT EXISTS Genres (
        GenreID INTEGER PRIMARY KEY AUTOINCREMENT,
        GenreName VARCHAR(50) NOT NULL UNIQUE
    );

    -- MoviesGenres junction table
    CREATE TABLE IF NOT EXISTS MoviesGenres (
        MovieID INTEGER,
        GenreID INTEGER,
        PRIMARY KEY (MovieID, GenreID),
        FOREIGN KEY (MovieID) REFERENCES Movies(MovieID),
        FOREIGN KEY (GenreID) REFERENCES Genres(GenreID)
    );

    -- UserWishlist table to store the wishlist relationship
    CREATE TABLE IF NOT EXISTS UserWishlist (
        UserID INTEGER,
        MovieID INTEGER,
        PRIMARY KEY (UserID, MovieID),
        FOREIGN KEY (UserID) REFERENCES Users(UserID),
        FOREIGN KEY (MovieID) REFERENCES Movies(MovieID)
    );

    -- Actors table
    CREATE TABLE IF NOT EXISTS Actors (
        ActorID INTEGER PRIMARY KEY AUTOINCREMENT,
        Name VARCHAR(255) NOT NULL,
        BirthDate DATE,
        Bio TEXT,
        CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    -- Directors table
    CREATE TABLE IF NOT EXISTS Directors (
        DirectorID INTEGER PRIMARY KEY AUTOINCREMENT,
        Name VARCHAR(255) NOT NULL,
        BirthDate DATE,
        Bio TEXT,
        CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    -- MoviesActors junction table
    CREATE TABLE IF NOT EXISTS MoviesActors (
        MovieID INTEGER,
        ActorID INTEGER,
        PRIMARY KEY (MovieID, ActorID),
        FOREIGN KEY (MovieID) REFERENCES Movies(MovieID),
        FOREIGN KEY (ActorID) REFERENCES Actors(ActorID)
    );

    -- MoviesDirectors junction table
    CREATE TABLE IF NOT EXISTS MoviesDirectors (
        MovieID INTEGER,
        DirectorID INTEGER,
        PRIMARY KEY (MovieID, DirectorID),
        FOREIGN KEY (MovieID) REFERENCES Movies(MovieID),
        FOREIGN KEY (DirectorID) REFERENCES Directors(DirectorID)
    );

    -- Reviews table
    CREATE TABLE IF NOT EXISTS Reviews (
        ReviewID INTEGER PRIMARY KEY AUTOINCREMENT,
        UserID INTEGER,
        MovieID INTEGER,
        Rating DECIMAL(2,1) CHECK (Rating BETWEEN 0 AND 10),
        ReviewText TEXT,
        CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (UserID) REFERENCES Users(UserID),
        FOREIGN KEY (MovieID) REFERENCES Movies(MovieID)
    );

    CREATE TABLE IF NOT EXISTS MovieRentalRecords (
        RentalID INTEGER PRIMARY KEY AUTOINCREMENT,
        UserID INTEGER NOT NULL,
        MovieID INTEGER NOT NULL,
        RentalDate DATE NOT NULL,
        ReturnDate DATE,
        RentalPrice DECIMAL(5, 2) NOT NULL,
        FOREIGN KEY (UserID) REFERENCES Users(UserID),
        FOREIGN KEY (MovieID) REFERENCES Movies(MovieID)
    );
    "#;

    conn.execute_batch(sql).unwrap();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search/<name>")]
fn search(name: &str, state: &State<AppState>) -> String {
    let conn = state.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Movies WHERE Title LIKE ?").unwrap();
    let mut rows = stmt.query([format!("%{}%", name)]).unwrap();
    let mut results = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        results.push(row.get::<usize, String>(1).unwrap());
    }
    format!("Search results: {:?}", results)
}

#[get("/browse/<id>")]
fn browse(id: i32, state: &State<AppState>) -> String {
    let conn = state.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Movies WHERE MovieID = ?").unwrap();
    let row = stmt.query_row([id], |row| row.get::<usize, String>(1)).unwrap();
    format!("Movie: {}", row)
}

#[post("/add-item", data = "<item>")]
fn add_item(item: Form<MovieRecord>, state: &State<AppState>) -> String {
    let conn = state.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO Movies (Title, Director, Starring, Details, Staffs, RentalPrice) VALUES (?, ?, ?, ?, ?, ?)",
        &[&item.title, &item.director, &item.starring, &item.details, &item.staffs, &item.rental_price],
    ).unwrap();
    format!("Added movie: {}", item.title)
}

#[post("/register", data = "<login>")]
fn register(login: Form<UserLogin>, state: &State<AppState>) -> String {
    let conn = state.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO Users (Username, PasswordHash, Email, DateOfBirth, GenderDescription) VALUES (?, ?, ?, ?, ?)",
        &[&login.username, &login.password_hash, &login.email, &login.date_of_birth, &login.gender_description],
    ).unwrap();
    format!("Registered user: {}", login.username)
}

#[post("/login", data = "<login>")]
fn login(login: Form<UserLogin>, state: &State<AppState>) -> String {
    let conn = state.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Users WHERE Username = ? AND PasswordHash = ?").unwrap();
    let row = stmt.query_row([&login.username, &login.password_hash], |row| row.get::<usize, String>(1)).unwrap();
    format!("Logged in user: {}", row)
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
    let conn = Connection::open("movie_rental.duckdb").unwrap();
    initialize_db(&conn);

    let app_state = AppState {
        conn: Mutex::new(conn),
    };

    rocket::build()
        .manage(app_state)
        .mount("/api", routes![index, search, browse, add_item, register, login])
        .mount("/", routes![serve_index, serve_login, serve_search])
}