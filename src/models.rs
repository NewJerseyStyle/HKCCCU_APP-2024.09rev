use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Actors)]
pub struct Actor {
    pub ActorID: i32,
    pub Name: String,
    pub BirthDate: Option<NaiveDate>,
    pub Bio: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub UpdatedAt: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Directors)]
pub struct Director {
    pub DirectorID: i32,
    pub Name: String,
    pub BirthDate: Option<NaiveDate>,
    pub Bio: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub UpdatedAt: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Genres)]
pub struct Genre {
    pub GenreID: i32,
    pub GenreName: String,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MovieRentalRecords)]
pub struct MovieRentalRecord {
    pub RentalID: i32,
    pub UserID: i32,
    pub MovieID: i32,
    pub RentalDate: NaiveDate,
    pub ReturnDate: Option<NaiveDate>,
    pub RentalPrice: Decimal,
}

#[derive(Debug, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Movies)]
pub struct Movie {
    pub MovieID: i32,
    pub Title: String,
    pub Director: String,
    pub Starring: String,
    pub Details: Option<String>,
    pub Staffs: Option<String>,
    pub RentalPrice: Option<Decimal>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MoviesActors)]
pub struct MovieActor {
    pub MovieID: i32,
    pub ActorID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MoviesDirectors)]
pub struct MovieDirector {
    pub MovieID: i32,
    pub DirectorID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MoviesGenres)]
pub struct MovieGenre {
    pub MovieID: i32,
    pub GenreID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Reviews)]
pub struct Review {
    pub ReviewID: i32,
    pub UserID: Option<i32>,
    pub MovieID: Option<i32>,
    pub Rating: Option<Decimal>,
    pub ReviewText: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub UpdatedAt: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::UserWishlist)]
pub struct UserWishlist {
    pub UserID: i32,
    pub MovieID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Users)]
pub struct User {
    pub UserID: i32,
    pub Username: String,
    pub PasswordHash: String,
    pub Email: String,
    pub DateOfBirth: NaiveDate,
    pub GenderDescription: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub UpdatedAt: Option<NaiveDateTime>,
}