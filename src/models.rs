use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{Movies, Users, MovieRentalRecords};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Movie {
    pub MovieID: i32,
    pub Title: String,
    pub Director: String,
    pub Starring: String,
    pub Details: Option<String>,
    pub Staffs: Option<String>,
    pub RentalPrice: Option<diesel::sql_types::Decimal>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub UserID: i32,
    pub Username: String,
    pub PasswordHash: String,
    pub Email: String,
    pub DateOfBirth: chrono::NaiveDate,
    pub GenderDescription: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct MovieRentalRecord {
    pub RentalID: i32,
    pub UserID: i32,
    pub MovieID: i32,
    pub RentalDate: chrono::NaiveDate,
    pub ReturnDate: Option<chrono::NaiveDate>,
    pub RentalPrice: diesel::sql_types::Decimal,
}

impl Movie {
    pub fn as_select() -> (Movies::MovieID, Movies::Title, Movies::Director, Movies::Starring, Movies::Details, Movies::Staffs, Movies::RentalPrice) {
        (Movies::MovieID, Movies::Title, Movies::Director, Movies::Starring, Movies::Details, Movies::Staffs, Movies::RentalPrice)
    }
}

impl User {
    pub fn as_select() -> (Users::UserID, Users::Username, Users::PasswordHash, Users::Email, Users::DateOfBirth, Users::GenderDescription) {
        (Users::UserID, Users::Username, Users::PasswordHash, Users::Email, Users::DateOfBirth, Users::GenderDescription)
    }
}

impl MovieRentalRecord {
    pub fn as_select() -> (MovieRentalRecords::RentalID, MovieRentalRecords::UserID, MovieRentalRecords::MovieID, MovieRentalRecords::RentalDate, MovieRentalRecords::ReturnDate, MovieRentalRecords::RentalPrice) {
        (MovieRentalRecords::RentalID, MovieRentalRecords::UserID, MovieRentalRecords::MovieID, MovieRentalRecords::RentalDate, MovieRentalRecords::ReturnDate, MovieRentalRecords::RentalPrice)
    }
}
