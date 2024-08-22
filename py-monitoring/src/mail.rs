use std::env;

use chrono::prelude::*;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(email: EMail) {
    let now = Utc::now();
    let now_formatted = format!(
        "{:0>2}-{:0>2} {:0>2}:{:0>2}",
        now.hour(),
        now.minute(),
        now.day(),
        now.month(),
    );

    let email_msg = Message::builder()
        .from(email.from.parse().unwrap())
        .to(email.to.parse().unwrap())
        .subject(format!("{} {}", now_formatted, email.subject))
        .header(ContentType::TEXT_PLAIN)
        .body(email.body)
        .unwrap();

    let creds = Credentials::new(
        email.from.to_owned(),
        env::var("MAIL_PW").unwrap().to_owned(),
    );

    let mailer = SmtpTransport::starttls_relay("smtp.office365.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email_msg) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

pub struct EMail<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub subject: &'a str,
    pub body: String,
}
