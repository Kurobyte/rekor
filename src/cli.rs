pub use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(short, long)]
    pub msg: String,

    #[clap(short, long)]
    pub time: Option<i64>,

    #[clap(short, long, default_value = "NO_PROJECT")]
    pub project: String,
}
