// Ext crates
use chrono::prelude::*;
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    {Message, SmtpTransport, Transport},
};

#[derive(Debug)]
pub struct MailInfo {
    pub from: String,
    pub from_pwd: String,
    pub to: String,
    pub subject: String,
}

impl MailInfo {
    pub fn new() -> MailInfo {
        MailInfo {
            from: String::new(),
            from_pwd: String::new(),
            to: String::new(),
            subject: String::new(),
        }
    }

    pub fn send(&self, body: String) {
        let now = Utc::now();
        let now_formatted = format!(
            "{:0>2}:{:0>2} {:0>2}-{:0>2}",
            now.hour(),
            now.minute(),
            now.day(),
            now.month(),
        );

        let email_msg = Message::builder()
            .from(self.from.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(format!("{} {}", now_formatted, &self.subject))
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .unwrap();

        let creds = Credentials::new(self.from.to_owned(), self.from_pwd.to_owned());

        let mailer = SmtpTransport::starttls_relay("smtp.office365.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email_msg) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => println!("Could not send email: {e:?}"),
        }
    }
}
