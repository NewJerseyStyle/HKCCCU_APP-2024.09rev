#[macro_use] extern crate rocket;
// #[macro_use] extern crate diesel;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use rocket::response::status::Custom;
use chrono::NaiveDate;
use validator::{Validate, ValidationError};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use serde::{Serialize, Deserialize};
use std::env;

mod schema;
mod models;

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Serialize, Deserialize)]
struct UserLogin {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserRegistration {
    #[validate(length(min = 3, max = 20))]
    username: String,
    #[validate(custom = "validate_password")]
    password: String,
    #[validate(email)]
    email: String,
    date_of_birth: NaiveDate,
    gender_description: Option<String>,
}

fn validate_password(password: &String) -> Result<(), ValidationError> {
    let re = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap();
    if re.is_match(password.as_str()) {
        Ok(())
    } else {
        Err(ValidationError::new("Password does not meet requirements"))
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    data: Option<T>,
    message: Option<String>,
}

#[get("/search/<name>")]
async fn search(name: &str, pool: &State<DbPool>) -> Result<Json<ApiResponse<Vec<models::Movie>>>, Custom<Json<ApiResponse<()>>>> {
    let conn = pool.get().map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Database connection error".to_string()),
    })))?;

    use schema::Movies::dsl::*;
    let results = Movies.filter(Title.like(format!("%{}%", name)))
        .load::<models::Movie>(&conn)
        .map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Database query error".to_string()),
        })))?;

    Ok(Json(ApiResponse {
        status: "success".to_string(),
        data: Some(results),
        message: None,
    }))
}

#[get("/browse/<id>")]
async fn browse(id: i32, pool: &State<DbPool>) -> Result<Json<ApiResponse<models::Movie>>, Custom<Json<ApiResponse<()>>>> {
    let conn = pool.get().map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Database connection error".to_string()),
    })))?;

    use schema::Movies::dsl::*;
    let movie = Movies.find(id)
        .first::<models::Movie>(&conn)
        .map_err(|_| Custom(Status::NotFound, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Movie not found".to_string()),
        })))?;

    Ok(Json(ApiResponse {
        status: "success".to_string(),
        data: Some(movie),
        message: None,
    }))
}

#[post("/wish-item/<movie_id>")]
async fn wish_item(movie_id: i32, pool: &State<DbPool>, claims: Claims) -> Result<Json<ApiResponse<()>>, Custom<Json<ApiResponse<()>>>> {
    let conn = pool.get().map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Database connection error".to_string()),
    })))?;

    use schema::UserWishlist::dsl::*;
    let wish = models::UserWishlist {
        user_id: claims.sub.parse().unwrap(),
        movie_id,
    };

    diesel::insert_into(UserWishlist)
        .values(&wish)
        .execute(&conn)
        .map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Failed to add to wishlist".to_string()),
        })))?;

    Ok(Json(ApiResponse {
        status: "success".to_string(),
        data: None,
        message: Some("Added to wishlist".to_string()),
    }))
}

#[post("/rent-item", data = "<item>")]
async fn rent_item(item: Json<MovieRentalRecord>, pool: &State<DbPool>, _claims: Claims) -> Result<Json<ApiResponse<()>>, Custom<Json<ApiResponse<()>>>> {
    let conn = pool.get().map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Database connection error".to_string()),
    })))?;

    diesel::insert_into(schema::MovieRentalRecords::table)
        .values(&item.into_inner())
        .execute(&conn)
        .map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Failed to add rental record".to_string()),
        })))?;

    Ok(Json(ApiResponse {
        status: "success".to_string(),
        data: None,
        message: Some("Rental record added".to_string()),
    }))
}

#[post("/register", data = "<data>")]
async fn register(data: Json<UserRegistration>, pool: &State<DbPool>) -> Result<Json<ApiResponse<()>>, Custom<Json<ApiResponse<()>>>> {
    if let Err(errors) = data.validate() {
        return Err(Custom(Status::BadRequest, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some(format!("Validation error: {:?}", errors)),
        })));
    }

    let conn = pool.get().map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Database connection error".to_string()),
    })))?;

    let hashed_password = hash(&data.password, DEFAULT_COST).map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Password hashing error".to_string()),
    })))?;

    let new_user = models::User {
        username: &data.username,
        password_hash: &hashed_password,
        email: &data.email,
        date_of_birth: data.date_of_birth,
        gender_description: data.gender_description.as_deref(),
    };

    diesel::insert_into(schema::Users::table)
        .values(&new_user)
        .execute(&conn)
        .map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Failed to register user".to_string()),
        })))?;

    Ok(Json(ApiResponse {
        status: "success".to_string(),
        data: None,
        message: Some("User registered successfully".to_string()),
    }))
}

#[post("/login", data = "<login>")]
async fn login(login: Json<UserLogin>, pool: &State<DbPool>) -> Result<Json<ApiResponse<LoginResponse>>, Custom<Json<ApiResponse<()>>>> {
    let conn = pool.get().map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Database connection error".to_string()),
    })))?;

    use schema::Users::dsl::*;
    let user = users.filter(Username.eq(&login.username))
        .first::<models::User>(&conn)
        .map_err(|_| Custom(Status::Unauthorized, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Invalid credentials".to_string()),
        })))?;

    if verify(&login.password, &user.password_hash).map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Password verification error".to_string()),
    })))? {
        let claims = Claims {
            sub: user.user_id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
            .map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
                status: "error".to_string(),
                data: None,
                message: Some("Token generation error".to_string()),
            })))?;

        Ok(Json(ApiResponse {
            status: "success".to_string(),
            data: Some(LoginResponse { token }),
            message: None,
        }))
    } else {
        Err(Custom(Status::Unauthorized, Json(ApiResponse {
            status: "error".to_string(),
            data: None,
            message: Some("Invalid credentials".to_string()),
        })))
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    rocket::build()
        .mount("/api", routes![search, browse, wish_item, rent_item, register, login])
        .manage(pool)
}
