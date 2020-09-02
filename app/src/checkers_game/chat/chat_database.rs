use diesel::prelude::*;
use crate::checkers_game::chat::chat_models;


pub fn insert_new_message(name: String, m: String, conn: &PgConnection)
{
    use crate::schema::checkers_game_chat::dsl::*;

    let new_message = chat_models::ChatMessage
    {
        user_name: name,
        message: m,
    };

    match diesel::insert_into(checkers_game_chat).values(&new_message).execute(conn)
    {
        Ok(_) => (),
        Err(_) => println!("Error to save message in database!!!")
    }
}


pub fn extract_chat_log(conn: &PgConnection) -> Result<Option<Vec<chat_models::ChatMessageResponse>>, diesel::result::Error>
{
    use crate::schema::checkers_game_chat::dsl::*;

    let all_messages = checkers_game_chat
        .load::<chat_models::ChatMessageResponse>(conn)
        .optional()?;
    Ok(all_messages)
}