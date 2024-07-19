use diesel::prelude::*;
use diesel::sql_types::*;

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: NaiveDate,
    pub gender_description: Option<String>,
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "movies"]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub director: String,
    pub starring: Vec<String>,
    pub details: Option<String>,
    pub staffs: Option<Vec<String>>,
    pub rental_price: Option<f64>,
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "movie_rentals"]
pub struct MovieRental {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub rental_date: NaiveDate,
}

#[derive(Queryable, Identifiable, Debug)]
#[table_name = "user_wishes"]
pub struct UserWish {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
}
