# HKCCCU_APP-2024.09rev
Revamp school project of mobile application a Blockbuster like online video rental shop mobile application.

This project is a Rust-based web application using Rocket framework and DuckDB for a movie rental system.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- DuckDB

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/movie-rental-api.git
   cd movie-rental-api
   ```

2. Build release:
   ```
   cargo build -release
   # cargo build --target x86_64-pc-windows-gnu
   ```

## Running the Application

To run the application in development mode:

```
cargo run
```

The server will start at `http://localhost:8000` by default.

## API Endpoints

- `GET /api`: Hello World endpoint
- `GET /api/search/<name>`: Search for movies
- `GET /api/browse/<id>`: Browse movie by ID
- `POST /api/add-item`: Add a new movie
- `POST /api/register`: Register a new user
- `POST /api/login`: User login
- `GET /home`: Serve index page
- `GET /`: Serve login page
- `GET /search`: Serve search page

## Testing

To run all tests:

```
cargo test
```

### API Testing

You can use tools like `curl` or Postman to test the API endpoints. Here are some example `curl` commands:

1. Search for a movie:
   ```
   curl http://localhost:8000/api/search/Matrix
   ```

2. Add a new movie:
   ```
   curl -X POST -H "Content-Type: application/x-www-form-urlencoded" -d "title=New%20Movie&director=John%20Doe&starring=Jane%20Doe&details=Great%20movie&staffs=Crew&rental_price=9.99" http://localhost:8000/api/add-item
   ```

## Database

The application uses DuckDB, a file-based database. The database file is created as `movie_rental.duckdb` in the project root directory.
