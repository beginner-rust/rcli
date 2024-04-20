use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv,or convert csv to other fomart")]
    Csv(CvsOpts),
}

#[derive(Parser, Debug)]
pub struct CvsOpts {
    #[arg(short, long, value_parser=verify_file_exists)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file_exists(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err("file not found".into())
    }
}
