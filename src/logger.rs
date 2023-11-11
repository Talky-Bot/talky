use chrono::Local;
use tokio::{fs, io::AsyncWriteExt, sync::Mutex};

// List all possible log types
pub enum LogType {
    Info,
    Warning,
    Error
}

// Make the struct for the logger, this keeps the file open and allows us to write to it without reopening it every time
pub struct Logger {
    file: Mutex<fs::File>
}

// This is the functions for the logger struct
impl Logger {
    // The new function is similar to the __init__ function in python
    pub async fn new() -> tokio::io::Result<Logger> {
        Ok(
            Logger {
                // Check if the file exists byt getting the metadata of the file. If it doesn't exist, create it
                file: {
                    if fs::metadata("./logs/").await.is_err() {
                        fs::create_dir("./logs/").await?;
                    }
    
                    if fs::metadata("./logs/latest.log").await.is_ok() {
                        let now = Local::now();
                        fs::rename("./logs/latest.log", format!("./logs/{}.log", now.format("%Y-%m-%d %H-%M-%S"))).await?;
                    }
    
                    fs::File::create("./logs/latest.log").await?;
                    // Mutex is used to safly write to the variable across multiple threads
                    Mutex::new(fs::OpenOptions::new().append(true).open("./logs/latest.log").await?)
                }
            }
        )
    }

    // The log function is used to write to the file and print the colored output to the console
    pub async fn log(&self, log_type: LogType, message: &str) -> Result<(), tokio::io::Error> {
        let time = Local::now().format("[%Y-%m-%d %H:%M:%S]");
        let log_string: String;
        let mut file_lock = self.file.lock().await;

        // Match 
        match log_type {
            LogType::Info => {
                log_string = format!("{} [INFO] {}\n", time, message);
                file_lock.write_all(log_string.as_bytes()).await.unwrap();
                print!("{}", log_string);
            },
            LogType::Warning => {
                log_string = format!("{} [WARNING] {}\n", time, message);
                file_lock.write_all(log_string.as_bytes()).await.unwrap();
                print!("\x1b[33m{}\x1b[0m", log_string);
            },
            LogType::Error => {
                log_string = format!("{} [ERROR] {}\n", time, message);
                file_lock.write_all(log_string.as_bytes()).await.unwrap();
                print!("\x1b[31m{}\x1b[0m", log_string);
            }
        };
        file_lock.flush().await?;
        Ok(())
    }
}
