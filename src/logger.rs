use chrono::{DateTime, Local};
use tokio::{fs, io::AsyncWriteExt, sync::Mutex};

use crate::error::Error;

pub enum LogType {
    Info,
    Warning,
    Error,
}

pub struct Logger {
    file: Mutex<fs::File>,
}

impl Logger {
    pub async fn new() -> tokio::io::Result<Logger> {
        Ok(Logger {
            file: {
                if let Err(_) = fs::metadata("./logs/").await {
                    fs::create_dir("./logs/").await?;
                }

                if let Ok(metadata) = fs::metadata("./logs/latest.log").await {
                    fs::rename(
                        "./logs/latest.log",
                        format!("./logs/{}.log", {
                            let datetime: DateTime<Local> = metadata.created().unwrap().into();
                            datetime.format("%Y-%m-%d %H-%M-%S").to_string()
                        }),
                    )
                    .await?;
                }

                fs::File::create("./logs/latest.log").await?;
                Mutex::new(
                    fs::OpenOptions::new()
                        .append(true)
                        .open("./logs/latest.log")
                        .await?,
                )
            },
        })
    }

    pub async fn log(&self, log_type: LogType, message: &str) -> Result<(), Error> {
        let time = Local::now().format("[%Y-%m-%d %H:%M:%S]");
        let log_string: String;
        let mut file_lock = self.file.lock().await;

        // Match
        match log_type {
            LogType::Info => {
                log_string = format!("{} [INFO] {}\n", time, message);
                file_lock.write_all(log_string.as_bytes()).await.unwrap();
                print!("{}", log_string);
            }
            LogType::Warning => {
                log_string = format!("{} [WARNING] {}\n", time, message);
                file_lock.write_all(log_string.as_bytes()).await.unwrap();
                print!("\x1b[33m{}\x1b[0m", log_string);
            }
            LogType::Error => {
                log_string = format!("{} [ERROR] {}\n", time, message);
                file_lock.write_all(log_string.as_bytes()).await.unwrap();
                print!("\x1b[31m{}\x1b[0m", log_string);
            }
        };
        file_lock.flush().await?;
        Ok(())
    }

    pub async fn log_error(&self, error: Error) -> Result<(), Error> {
        let time = Local::now().format("[%Y-%m-%d %H:%M:%S]");
        let log_string: String;
        let mut file_lock = self.file.lock().await;

        log_string = format!("{} [WARNING] {}\n", time, error.to_string());
        file_lock.write_all(log_string.as_bytes()).await?;
        print!("\x1b[33m{}\x1b[0m", log_string);

        Ok(())
    }
}
