use clap::Parser;


#[derive(Parser, Debug)]
#[command(name="rcli",version,author,about,long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,

}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(name="csv",about ="show csv,or convert csv to other fomart")]
    Csv(CvsOpts),
}

#[derive(Parser, Debug)]
struct CvsOpts {

    #[arg(short,long)]
    input: String,

    #[arg(short,long,default_value="output.json")]
    output: String,

    #[arg(short,long,default_value_t = ',')]
    delimiter: char,

    #[arg(short,long,default_value_t = true)]
    header: bool,

}

fn main() {
    let opts = Opts::parse();
    panic!("{:?}",opts)
}
