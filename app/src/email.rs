use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};


const SENDER_EMAIL: &str = "zz.roman.a.shushakov@gmail.com";
const SENDER_PASSWORD: &str = "ltajhvfwbz";
const SENDER_DOMAIN: &str = "smtp.gmail.com";
const RECEIVER_EMAIL: &str = "roman.a.shushakov@mail.ru";


pub fn send_email(user_name: String)
{
       let email = Email::builder()
        .to(RECEIVER_EMAIL)
        .from(SENDER_EMAIL)
        .subject("new_user_registered")
        .text(format!("User '{}' was successfully registered.", user_name))
        .build()
        .unwrap();

    let creds = Credentials::new(
        SENDER_EMAIL.to_owned(),
        SENDER_PASSWORD.to_owned(),
    );


    let mut mailer = SmtpClient::new_simple(SENDER_DOMAIN)
        .unwrap()
        .credentials(creds)
        .transport();

    let _ = mailer.send(email.into());
}
