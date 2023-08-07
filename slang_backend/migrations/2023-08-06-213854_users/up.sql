-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    displayname TEXT,
    icon_url TEXT NOT NULL,
    friends INTEGER ARRAY,
    blocked INTEGER ARRAY
);