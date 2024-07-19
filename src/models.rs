use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::deserialize::{FromSql, QueryableByName};
use diesel::serialize::{ToSql, Output};
use diesel::expression::AsExpression;
use diesel::mysql::{Mysql, MysqlValue};
use diesel::sql_types::Numeric;
use diesel::row::NamedRow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, FromSqlRow, AsExpression)]
#[diesel(sql_type = Numeric)]
pub struct SqlDecimal(pub Decimal);

// impl FromSql<diesel::sql_types::Numeric, Mysql> for SqlDecimal {
//     fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
//         let bytes = not_none!(bytes);
//         let s = std::str::from_utf8(bytes)?;
//         let decimal = rust_decimal::Decimal::from_str(s).map_err(|_| "Invalid Decimal".into())?;
//         Ok(SqlDecimal(decimal))
//     }
// }
impl FromSql<Numeric, Mysql> for SqlDecimal {
    fn from_sql(value: MysqlValue) -> diesel::deserialize::Result<Self> {
        let decimal = Decimal::from_sql(value)?;
        Ok(SqlDecimal(decimal))
    }
}

impl ToSql<Numeric, Mysql> for SqlDecimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> diesel::serialize::Result {
        // Convert Decimal to bytes or use the appropriate serialization method
        let bytes = self.0.serialize_to_bytes()?;
        
        // Write the bytes to the output
        out.write_all(&bytes)?;
        
        // Return IsNull::No to indicate that this value is not null
        Ok(IsNull::No)
    }
}

impl QueryableByName<Mysql> for SqlDecimal {
    fn build<R: NamedRow<Mysql>>(row: &R) -> diesel::deserialize::Result<Self> {
        let decimal: Decimal = Decimal::build(row)?;
        Ok(SqlDecimal(decimal))
    }
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Actors)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
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
#[diesel(check_for_backend(diesel::mysql::Mysql))]
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
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Genre {
    pub GenreID: i32,
    pub GenreName: String,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MovieRentalRecords)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MovieRentalRecord {
    pub RentalID: i32,
    pub UserID: i32,
    pub MovieID: i32,
    pub RentalDate: NaiveDate,
    pub ReturnDate: Option<NaiveDate>,
    pub RentalPrice: SqlDecimal,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Movies)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Movie {
    pub MovieID: i32,
    pub Title: String,
    pub Director: String,
    pub Starring: String,
    pub Details: Option<String>,
    pub Staffs: Option<String>,
    pub RentalPrice: Option<SqlDecimal>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MoviesActors)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MovieActor {
    pub MovieID: i32,
    pub ActorID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MoviesDirectors)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MovieDirector {
    pub MovieID: i32,
    pub DirectorID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::MoviesGenres)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct MovieGenre {
    pub MovieID: i32,
    pub GenreID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Reviews)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Review {
    pub ReviewID: i32,
    pub UserID: Option<i32>,
    pub MovieID: Option<i32>,
    pub Rating: Option<SqlDecimal>,
    pub ReviewText: Option<String>,
    pub CreatedAt: Option<NaiveDateTime>,
    pub UpdatedAt: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::UserWishlist)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserWishlist {
    pub UserID: i32,
    pub MovieID: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::Users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
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
