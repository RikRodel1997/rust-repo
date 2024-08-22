mod input;
mod mail;
use mail::EMail;

use dotenv::dotenv;
use std::process::{Command, Stdio};

fn main() {
    dotenv().ok();
    let mail_from = input::get_input("Enter your email to send notifications from: ");
    let mail_to = input::get_input("Enter the recipients email details for notifications: ");
    let mail_subject = input::get_input("Enter email subject for notification email: ");
    let process = input::get_input("Enter Python script path: ");

    loop {
        let child = Command::new("python3")
            .arg(process.as_str())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let err_out = child.wait_with_output().expect("Failed to wait on process");

        if !err_out.status.success() {
            let exception = String::from_utf8_lossy(&err_out.stderr).to_string();
            if exception.contains("KeyboardInterrupt") {
                println!("Python script was stopped due to a KeyboardInterrupt. Not restarting the script.");
                break;
            } else {
                println!("Python script ran into unexpected exception. Restarting the script.");
                let email = EMail {
                    from: &mail_from,
                    to: &mail_to,
                    subject: &mail_subject,
                    body: exception,
                };
                mail::send_email(email);
            }
        }
    }
}
