table! {
    checkers_game_chat (id) {
        id -> Int4,
        chat_room -> Varchar,
        user_name -> Varchar,
        message -> Varchar,
    }
}

table! {
    users_data (id) {
        id -> Varchar,
        user_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_superuser -> Bool,
        is_active -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    checkers_game_chat,
    users_data,
);
