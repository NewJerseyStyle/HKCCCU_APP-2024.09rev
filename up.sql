-- Users table
CREATE TABLE Users (
    UserID INT PRIMARY KEY AUTO_INCREMENT,
    Username VARCHAR(50) NOT NULL UNIQUE,
    PasswordHash VARCHAR(255) NOT NULL,
    Email VARCHAR(100) NOT NULL UNIQUE,
    DateOfBirth DATE NOT NULL,
    GenderDescription VARCHAR(50),
    CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Movies table
CREATE TABLE Movies (
    MovieID INT PRIMARY KEY AUTO_INCREMENT,
    Title VARCHAR(255) NOT NULL,
    Director VARCHAR(255) NOT NULL,
    Starring TEXT NOT NULL,
    Details TEXT,
    Staffs TEXT,
    RentalPrice DECIMAL(5, 2) -- Price for renting the movie, allowing for two decimal places
);

-- Genres table
CREATE TABLE Genres (
    GenreID INT PRIMARY KEY AUTO_INCREMENT,
    GenreName VARCHAR(50) NOT NULL UNIQUE
);

-- MoviesGenres junction table
CREATE TABLE MoviesGenres (
    MovieID INT,
    GenreID INT,
    PRIMARY KEY (MovieID, GenreID),
    FOREIGN KEY (MovieID) REFERENCES Movies(MovieID),
    FOREIGN KEY (GenreID) REFERENCES Genres(GenreID)
);

-- UserWishlist table to store the wishlist relationship
CREATE TABLE UserWishlist (
    UserID INT,
    MovieID INT,
    PRIMARY KEY (UserID, MovieID),
    FOREIGN KEY (UserID) REFERENCES Users(UserID),
    FOREIGN KEY (MovieID) REFERENCES Movies(MovieID)
);

-- Actors table
CREATE TABLE Actors (
    ActorID INT PRIMARY KEY AUTO_INCREMENT,
    Name VARCHAR(255) NOT NULL,
    BirthDate DATE,
    Bio TEXT,
    CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Directors table
CREATE TABLE Directors (
    DirectorID INT PRIMARY KEY AUTO_INCREMENT,
    Name VARCHAR(255) NOT NULL,
    BirthDate DATE,
    Bio TEXT,
    CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- MoviesActors junction table
CREATE TABLE MoviesActors (
    MovieID INT,
    ActorID INT,
    PRIMARY KEY (MovieID, ActorID),
    FOREIGN KEY (MovieID) REFERENCES Movies(MovieID),
    FOREIGN KEY (ActorID) REFERENCES Actors(ActorID)
);

-- MoviesDirectors junction table
CREATE TABLE MoviesDirectors (
    MovieID INT,
    DirectorID INT,
    PRIMARY KEY (MovieID, DirectorID),
    FOREIGN KEY (MovieID) REFERENCES Movies(MovieID),
    FOREIGN KEY (DirectorID) REFERENCES Directors(DirectorID)
);

-- Reviews table
CREATE TABLE Reviews (
    ReviewID INT PRIMARY KEY AUTO_INCREMENT,
    UserID INT,
    MovieID INT,
    Rating DECIMAL(2,1) CHECK (Rating BETWEEN 0 AND 10),
    ReviewText TEXT,
    CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (UserID) REFERENCES Users(UserID),
    FOREIGN KEY (MovieID) REFERENCES Movies(MovieID)
);

CREATE TABLE MovieRentalRecords (
    RentalID INT PRIMARY KEY AUTO_INCREMENT,
    UserID INT NOT NULL,
    MovieID INT NOT NULL,
    RentalDate DATE NOT NULL,
    ReturnDate DATE,
    RentalPrice DECIMAL(5, 2) NOT NULL,
    FOREIGN KEY (UserID) REFERENCES Users(UserID),
    FOREIGN KEY (MovieID) REFERENCES Movies(MovieID)
);
