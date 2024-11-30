use std::process::Command;

pub enum DbTypes {
    MySql,
}

pub struct Db {
    pub db_type: DbTypes,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Db {
    pub fn get_tables(&self) {
        let query = format!(
            "mysql -h{} -P{} -u{} -p{} -D{} -e \"SHOW TABLES\"",
            &self.host, &self.port, &self.user, &self.password, &self.database
        );

        let out = Command::new("sh")
            .arg("-c")
            .arg(query)
            .output()
            .expect("Failed to execute command");

        if out.status.success() {
            let stdout = String::from_utf8(out.stdout).expect("Failed to parse output");
            println!("stdout: {}", stdout);
        } else {
            let stderr = String::from_utf8(out.stderr).expect("Failed to parse error output");
            eprintln!("stderr: {}", stderr);
        }
    }
}
