-- Add down migration script here

CREATE TABLE
    IF NOT EXISTS note (
           id CHAR(36) PRIMARY KEY NOT NULL,
        title VARCHAR(255) NOT NULL UNIQUE,
        author_id: Integer,
        content TEXT NOT NULL,
        category VARCHAR(100),
        published BOOLEAN NOT NULL DEFAULT FALSE
    );

CREATE TABLE
    IF NOT EXISTS author (
        id CHAR(36) PRIMARY KEY NOT NULL,
        `name` VARCHAR(255) NOT NULL UNIQUE,
        email VARCHAR(100),
        status BOOLEAN NOT NULL DEFAULT TRUE
    );