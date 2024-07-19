use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    pub actor_id: i32,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Director {
    pub director_id: i32,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub genre_id: i32,
    pub genre_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieRentalRecord {
    pub rental_id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub rental_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub rental_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub movie_id: i32,
    pub title: String,
    pub director: String,
    pub starring: String,
    pub details: Option<String>,
    pub staffs: Option<String>,
    pub rental_price: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieActor {
    pub movie_id: i32,
    pub actor_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieDirector {
    pub movie_id: i32,
    pub director_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieGenre {
    pub movie_id: i32,
    pub genre_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub review_id: i32,
    pub user_id: Option<i32>,
    pub movie_id: Option<i32>,
    pub rating: Option<Decimal>,
    pub review_text: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWishlist {
    pub user_id: i32,
    pub movie_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
    pub gender_description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
