-- Your SQL goes here
CREATE TABLE checkers_game_chat (
  id SERIAL PRIMARY KEY,
  chat_room VARCHAR NOT NULL,
  user_name VARCHAR NOT NULL,
  message VARCHAR NOT NULL
)