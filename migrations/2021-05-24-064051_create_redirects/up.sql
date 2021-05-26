CREATE TABLE redirects(
        id SERIAL PRIMARY KEY,
        alias VARCHAR NOT NULL,
        url VARCHAR NOT NULL
)
