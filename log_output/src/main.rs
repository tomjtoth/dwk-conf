use std::{env, fs::File, io::Write, time::Duration};

use time::UtcDateTime;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let id = Uuid::new_v4();

    let log_path = env::var("LOG_PATH").unwrap_or(String::from("logs/output.log"));

    let mut file = File::options()
        .append(true)
        .create(true)
        .open(log_path)
        .unwrap();
    loop {
        let _ = writeln!(&mut file, "{}: {}", UtcDateTime::now(), id);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
