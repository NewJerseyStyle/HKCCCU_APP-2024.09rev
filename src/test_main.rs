#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};

    fn setup_rocket() -> rocket::Rocket<rocket::Build> {
        let conn = Connection::open("test_movie_rental.duckdb").unwrap();
        initialize_db(&conn);

        let app_state = AppState {
            conn: Mutex::new(conn),
        };

        rocket::build()
            .manage(app_state)
            .mount("/api", routes![index, search, browse, add_item, register, login])
            .mount("/", routes![serve_index, serve_login, serve_search])
    }

    #[test]
    fn test_index() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/api").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn test_search() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/api/search/Test").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more specific assertions based on your expected search results
    }

    #[test]
    fn test_browse() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/api/browse/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more specific assertions based on your expected browse results
    }

    #[test]
    fn test_add_item() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.post("/api/add-item")
            .header(ContentType::Form)
            .body("title=Test%20Movie&director=Test%20Director&starring=Test%20Actor&details=Test%20Details&staffs=Test%20Staff&rental_price=9.99")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Added movie: Test Movie"));
    }

    #[test]
    fn test_register() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.post("/api/register")
            .header(ContentType::Form)
            .body("username=testuser&password_hash=testhash&email=test@example.com&date_of_birth=1990-01-01&gender_description=Test")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Registered user: testuser"));
    }

    #[test]
    fn test_login() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        // First, register a user
        client.post("/api/register")
            .header(ContentType::Form)
            .body("username=testuser&password_hash=testhash&email=test@example.com&date_of_birth=1990-01-01&gender_description=Test")
            .dispatch();

        // Then, try to log in
        let response = client.post("/api/login")
            .header(ContentType::Form)
            .body("username=testuser&password_hash=testhash")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().unwrap().contains("Logged in user: testuser"));
    }

    #[test]
    fn test_serve_index() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/home").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more specific assertions if needed
    }

    #[test]
    fn test_serve_login() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more specific assertions if needed
    }

    #[test]
    fn test_serve_search() {
        let client = Client::tracked(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/search").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more specific assertions if needed
    }
}