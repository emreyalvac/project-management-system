use lettre::{SmtpTransport, Transport, Message};

use std::env;
use lettre::transport::smtp::authentication::Credentials;
use async_trait::async_trait;
use domain::email::email::EmailSend;
use lettre::message::{SinglePart, MultiPart};
use lettre::message::header::ContentType;

#[async_trait]
pub trait TEmail {
    async fn send_email(&self, email: EmailSend) -> Result<bool, bool>;
}

pub struct Email {}

#[async_trait]
impl TEmail for Email {
    // TODO : Test html for content
    // TODO: SMTP Server
    async fn send_email(&self, email: EmailSend) -> Result<bool, bool> {
        let email = Message::builder()
            // TODO: Set from static
            .to(email.to.parse().unwrap())
            .from(email.from.parse().unwrap())
            .subject(email.subject)
            .multipart(MultiPart::mixed().multipart(MultiPart::related().singlepart(SinglePart::eight_bit().header(ContentType::html()).body(email.body))))
            .unwrap();

        // Credentials
        let env_username = env::var("GMAIL_USERNAME").unwrap();
        let env_password = env::var("GMAIL_PASSWORD").unwrap();
        let creds = Credentials::new(env_username, env_password);

        // Gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        let result = mailer.send(&email);
        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_email_test() {
        let email = Email {};
        let handler = email.send_email(EmailSend { from: "rusttestemail12@gmail.com".parse().unwrap(), to: "rusttestemail12@gmail.com".parse().unwrap(), subject: "test".parse().unwrap(), body: "body".parse().unwrap() }).await;
        assert_eq!(true, handler.unwrap())
    }
}
