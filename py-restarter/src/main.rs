mod input;
mod mail_file;
mod mail_info;

// Std Library
use std::{
    path::Path,
    process::{Command, Stdio},
};

// Custom crates
use input::get_input;
use mail_file::{decrypt_from_file, encrypt_to_file};
use mail_info::MailInfo;

fn main() {
    let mut mail_info = MailInfo::new();
    let has_mail_info_bin_file = get_input("Do you already have a mail-info.bin file? (Y/N)");

    if has_mail_info_bin_file.to_uppercase().as_str() == "Y" {
        let mut mail_info_path = get_input("Please enter the path to your mail-info.bin file.");
        while !Path::new(&mail_info_path).exists() {
            println!("There's no mail-info.bin at {}", mail_info_path);
            mail_info_path = get_input("Please enter the path to your mail-info.bin file.")
        }

        let password = get_input("Enter the password with which you encrypted the file:");
        match decrypt_from_file(&password, &mail_info_path) {
            Ok(info) => {
                println!("Succesfully decrypted mail details");
                mail_info = info;
            }
            Err(err) => println!("Error while decrypting {:?}", err),
        }
    } else if has_mail_info_bin_file.to_uppercase().as_str() == "N" {
        println!("You specified that you don't have a mail-info.bin file yet. We'll make it now by answering these questions");
        match encrypt_to_file(
            &get_input("Enter a password to encrypt the file:"),
            &prompt_email_details().as_bytes(),
            "mail-info.bin",
        ) {
            Ok(()) => println!("Succesfully encrypted mail details to file."),
            Err(err) => println!("Unsuccesfully encrypted file: {}", err),
        }
    } else {
        println!("Invalid input, please use Y (y) or N (n) next time.");
        return main();
    }

    let process = input::get_input("Enter Python script path you want to automatically restart: ");
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
                mail_info.send(exception);
            }
        }
    }
}

fn prompt_email_details() -> String {
    format!(
        "FROM={}\nFROM_PWD={}\nTO={}\nSUBJECT={}",
        get_input("Enter sender's email address:"),
        get_input("Enter sender's email password:"),
        get_input("Enter recipient's email address:"),
        get_input("Enter subject for notification email:")
    )
}
