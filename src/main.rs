#[macro_use] extern crate rocket;
// #[macro_use] extern crate diesel;
use rocket::serde::json::Json;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use rocket::State;
use rocket::response::status::Custom;
use chrono::NaiveDate;
use validator::{Validate};
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
    password: String,
    #[validate(email)]
    email: String,
    date_of_birth: NaiveDate,
    gender_description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MovieRentalRecord {
    UserID: i32,
    MovieID: i32,
    RentalDate: NaiveDate
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the token from the Authorization header
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) if token.starts_with("Bearer ") => {
                let token = token.trim_start_matches("Bearer ").trim();
                
                // Decode and validate the token
                let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let key = DecodingKey::from_secret(secret.as_bytes());
                match decode::<Claims>(token, &key, &Validation::default()) {
                    Ok(token_data) => Outcome::Success(token_data.claims),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
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
    let conn = &mut pool.get().map_err(|_| Custom(Status::InternalServerError, Json(ApiResponse {
        status: "error".to_string(),
        data: None,
        message: Some("Database connection error".to_string()),
    })))?;

    use crate::schema::Movies::dsl::*;
    let results = Movies
        .filter(Title.like(format!("%{}%", name)))
        .select(models::Movie::as_select())
        .load(conn)
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
        UserID: claims.sub.parse().unwrap(),
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
    let user = Users.filter(Username.eq(&login.username))
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
            sub: user.UserID.to_string(),
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
