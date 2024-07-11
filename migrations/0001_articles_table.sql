CREATE TABLE IF NOT EXISTS articles (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    title VARCHAR(150) NOT NULL,
    date_of_publication DATE NOT NULL,
    category VARCHAR(50) NOT NULL,
    description VARCHAR(150),
    content TEXT NOT NULL
);
