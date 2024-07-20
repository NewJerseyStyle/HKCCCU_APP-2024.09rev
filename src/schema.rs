// @generated automatically by Diesel CLI.

diesel::table! {
    /// Representation of the `Actors` table.
    ///
    /// (Automatically generated by Diesel.)
    Actors (ActorID) {
        /// The `ActorID` column of the `Actors` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        ActorID -> Integer,
        /// The `Name` column of the `Actors` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        Name -> Varchar,
        /// The `BirthDate` column of the `Actors` table.
        ///
        /// Its SQL type is `Nullable<Date>`.
        ///
        /// (Automatically generated by Diesel.)
        BirthDate -> Nullable<Date>,
        /// The `Bio` column of the `Actors` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        Bio -> Nullable<Text>,
        /// The `CreatedAt` column of the `Actors` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        CreatedAt -> Nullable<Timestamp>,
        /// The `UpdatedAt` column of the `Actors` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        UpdatedAt -> Nullable<Timestamp>,
    }
}

diesel::table! {
    /// Representation of the `Directors` table.
    ///
    /// (Automatically generated by Diesel.)
    Directors (DirectorID) {
        /// The `DirectorID` column of the `Directors` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        DirectorID -> Integer,
        /// The `Name` column of the `Directors` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        Name -> Varchar,
        /// The `BirthDate` column of the `Directors` table.
        ///
        /// Its SQL type is `Nullable<Date>`.
        ///
        /// (Automatically generated by Diesel.)
        BirthDate -> Nullable<Date>,
        /// The `Bio` column of the `Directors` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        Bio -> Nullable<Text>,
        /// The `CreatedAt` column of the `Directors` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        CreatedAt -> Nullable<Timestamp>,
        /// The `UpdatedAt` column of the `Directors` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        UpdatedAt -> Nullable<Timestamp>,
    }
}

diesel::table! {
    /// Representation of the `Genres` table.
    ///
    /// (Automatically generated by Diesel.)
    Genres (GenreID) {
        /// The `GenreID` column of the `Genres` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        GenreID -> Integer,
        /// The `GenreName` column of the `Genres` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 50]
        GenreName -> Varchar,
    }
}

diesel::table! {
    /// Representation of the `MovieRentalRecords` table.
    ///
    /// (Automatically generated by Diesel.)
    MovieRentalRecords (RentalID) {
        /// The `RentalID` column of the `MovieRentalRecords` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        RentalID -> Integer,
        /// The `UserID` column of the `MovieRentalRecords` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        UserID -> Integer,
        /// The `MovieID` column of the `MovieRentalRecords` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        MovieID -> Integer,
        /// The `RentalDate` column of the `MovieRentalRecords` table.
        ///
        /// Its SQL type is `Date`.
        ///
        /// (Automatically generated by Diesel.)
        RentalDate -> Date,
        /// The `ReturnDate` column of the `MovieRentalRecords` table.
        ///
        /// Its SQL type is `Nullable<Date>`.
        ///
        /// (Automatically generated by Diesel.)
        ReturnDate -> Nullable<Date>,
        /// The `RentalPrice` column of the `MovieRentalRecords` table.
        ///
        /// Its SQL type is `Decimal`.
        ///
        /// (Automatically generated by Diesel.)
        RentalPrice -> Decimal,
    }
}

diesel::table! {
    /// Representation of the `Movies` table.
    ///
    /// (Automatically generated by Diesel.)
    Movies (MovieID) {
        /// The `MovieID` column of the `Movies` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        MovieID -> Integer,
        /// The `Title` column of the `Movies` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        Title -> Varchar,
        /// The `Director` column of the `Movies` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        Director -> Varchar,
        /// The `Starring` column of the `Movies` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        Starring -> Text,
        /// The `Details` column of the `Movies` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        Details -> Nullable<Text>,
        /// The `Staffs` column of the `Movies` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        Staffs -> Nullable<Text>,
        /// The `RentalPrice` column of the `Movies` table.
        ///
        /// Its SQL type is `Nullable<Decimal>`.
        ///
        /// (Automatically generated by Diesel.)
        RentalPrice -> Nullable<Decimal>,
    }
}

diesel::table! {
    /// Representation of the `MoviesActors` table.
    ///
    /// (Automatically generated by Diesel.)
    MoviesActors (MovieID, ActorID) {
        /// The `MovieID` column of the `MoviesActors` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        MovieID -> Integer,
        /// The `ActorID` column of the `MoviesActors` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        ActorID -> Integer,
    }
}

diesel::table! {
    /// Representation of the `MoviesDirectors` table.
    ///
    /// (Automatically generated by Diesel.)
    MoviesDirectors (MovieID, DirectorID) {
        /// The `MovieID` column of the `MoviesDirectors` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        MovieID -> Integer,
        /// The `DirectorID` column of the `MoviesDirectors` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        DirectorID -> Integer,
    }
}

diesel::table! {
    /// Representation of the `MoviesGenres` table.
    ///
    /// (Automatically generated by Diesel.)
    MoviesGenres (MovieID, GenreID) {
        /// The `MovieID` column of the `MoviesGenres` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        MovieID -> Integer,
        /// The `GenreID` column of the `MoviesGenres` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        GenreID -> Integer,
    }
}

diesel::table! {
    /// Representation of the `Reviews` table.
    ///
    /// (Automatically generated by Diesel.)
    Reviews (ReviewID) {
        /// The `ReviewID` column of the `Reviews` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        ReviewID -> Integer,
        /// The `UserID` column of the `Reviews` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        UserID -> Nullable<Integer>,
        /// The `MovieID` column of the `Reviews` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        MovieID -> Nullable<Integer>,
        /// The `Rating` column of the `Reviews` table.
        ///
        /// Its SQL type is `Nullable<Decimal>`.
        ///
        /// (Automatically generated by Diesel.)
        Rating -> Nullable<Decimal>,
        /// The `ReviewText` column of the `Reviews` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        ReviewText -> Nullable<Text>,
        /// The `CreatedAt` column of the `Reviews` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        CreatedAt -> Nullable<Timestamp>,
        /// The `UpdatedAt` column of the `Reviews` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        UpdatedAt -> Nullable<Timestamp>,
    }
}

diesel::table! {
    /// Representation of the `UserWishlist` table.
    ///
    /// (Automatically generated by Diesel.)
    UserWishlist (UserID, MovieID) {
        /// The `UserID` column of the `UserWishlist` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        UserID -> Integer,
        /// The `MovieID` column of the `UserWishlist` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        MovieID -> Integer,
    }
}

diesel::table! {
    /// Representation of the `Users` table.
    ///
    /// (Automatically generated by Diesel.)
    Users (UserID) {
        /// The `UserID` column of the `Users` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        UserID -> Integer,
        /// The `Username` column of the `Users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 50]
        Username -> Varchar,
        /// The `PasswordHash` column of the `Users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 255]
        PasswordHash -> Varchar,
        /// The `Email` column of the `Users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 100]
        Email -> Varchar,
        /// The `DateOfBirth` column of the `Users` table.
        ///
        /// Its SQL type is `Date`.
        ///
        /// (Automatically generated by Diesel.)
        DateOfBirth -> Date,
        /// The `GenderDescription` column of the `Users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        #[max_length = 50]
        GenderDescription -> Nullable<Varchar>,
        /// The `CreatedAt` column of the `Users` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        CreatedAt -> Nullable<Timestamp>,
        /// The `UpdatedAt` column of the `Users` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        UpdatedAt -> Nullable<Timestamp>,
    }
}

diesel::joinable!(MovieRentalRecords -> Movies (MovieID));
diesel::joinable!(MovieRentalRecords -> Users (UserID));
diesel::joinable!(MoviesActors -> Actors (ActorID));
diesel::joinable!(MoviesActors -> Movies (MovieID));
diesel::joinable!(MoviesDirectors -> Directors (DirectorID));
diesel::joinable!(MoviesDirectors -> Movies (MovieID));
diesel::joinable!(MoviesGenres -> Genres (GenreID));
diesel::joinable!(MoviesGenres -> Movies (MovieID));
diesel::joinable!(Reviews -> Movies (MovieID));
diesel::joinable!(Reviews -> Users (UserID));
diesel::joinable!(UserWishlist -> Movies (MovieID));
diesel::joinable!(UserWishlist -> Users (UserID));

diesel::allow_tables_to_appear_in_same_query!(
    Actors,
    Directors,
    Genres,
    MovieRentalRecords,
    Movies,
    MoviesActors,
    MoviesDirectors,
    MoviesGenres,
    Reviews,
    UserWishlist,
    Users,
);
