// @generated automatically by Diesel CLI.

diesel::table! {
    Actors (ActorID) {
        ActorID -> Integer,
        #[max_length = 255]
        Name -> Varchar,
        BirthDate -> Nullable<Date>,
        Bio -> Nullable<Text>,
        CreatedAt -> Nullable<Timestamp>,
        UpdatedAt -> Nullable<Timestamp>,
    }
}

diesel::table! {
    Directors (DirectorID) {
        DirectorID -> Integer,
        #[max_length = 255]
        Name -> Varchar,
        BirthDate -> Nullable<Date>,
        Bio -> Nullable<Text>,
        CreatedAt -> Nullable<Timestamp>,
        UpdatedAt -> Nullable<Timestamp>,
    }
}

diesel::table! {
    Genres (GenreID) {
        GenreID -> Integer,
        #[max_length = 50]
        GenreName -> Varchar,
    }
}

diesel::table! {
    MovieRentalRecords (RentalID) {
        RentalID -> Integer,
        UserID -> Integer,
        MovieID -> Integer,
        RentalDate -> Date,
        ReturnDate -> Nullable<Date>,
        RentalPrice -> Decimal,
    }
}

diesel::table! {
    Movies (MovieID) {
        MovieID -> Integer,
        #[max_length = 255]
        Title -> Varchar,
        #[max_length = 255]
        Director -> Varchar,
        Starring -> Text,
        Details -> Nullable<Text>,
        Staffs -> Nullable<Text>,
        RentalPrice -> Nullable<Decimal>,
    }
}

diesel::table! {
    MoviesActors (MovieID, ActorID) {
        MovieID -> Integer,
        ActorID -> Integer,
    }
}

diesel::table! {
    MoviesDirectors (MovieID, DirectorID) {
        MovieID -> Integer,
        DirectorID -> Integer,
    }
}

diesel::table! {
    MoviesGenres (MovieID, GenreID) {
        MovieID -> Integer,
        GenreID -> Integer,
    }
}

diesel::table! {
    Reviews (ReviewID) {
        ReviewID -> Integer,
        UserID -> Nullable<Integer>,
        MovieID -> Nullable<Integer>,
        Rating -> Nullable<Decimal>,
        ReviewText -> Nullable<Text>,
        CreatedAt -> Nullable<Timestamp>,
        UpdatedAt -> Nullable<Timestamp>,
    }
}

diesel::table! {
    UserWishlist (UserID, MovieID) {
        UserID -> Integer,
        MovieID -> Integer,
    }
}

diesel::table! {
    Users (UserID) {
        UserID -> Integer,
        #[max_length = 50]
        Username -> Varchar,
        #[max_length = 255]
        PasswordHash -> Varchar,
        #[max_length = 100]
        Email -> Varchar,
        DateOfBirth -> Date,
        #[max_length = 50]
        GenderDescription -> Nullable<Varchar>,
        CreatedAt -> Nullable<Timestamp>,
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
