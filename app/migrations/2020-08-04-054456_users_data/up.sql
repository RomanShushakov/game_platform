-- Your SQL goes here
CREATE TABLE users_data (
  id VARCHAR NOT NULL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  is_superuser BOOLEAN NOT NULL DEFAULT 'f',
  is_active BOOLEAN NOT NULL DEFAULT 't'
)