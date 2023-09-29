use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "lru-s3-cache")]
#[command(author = "Fan Y. <hustyefan@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Cache S3 data in the local disk", long_about = None)]
pub struct CommandLineOption {
    /// Sets a data directory
    #[arg(short, long, value_name = "config")]
    pub config: String,
}
impl CommandLineOption {
    pub fn new() -> CommandLineOption {
        CommandLineOption::parse()
    }
}
