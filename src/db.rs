use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root:secret@localhost/movie_rental";
    MysqlConnection::establish(database_url)
        .expect("Error connecting to database")
}
