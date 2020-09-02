-- Your SQL goes here
CREATE TABLE checkers_game_chat (
  id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  message VARCHAR NOT NULL
)