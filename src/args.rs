#[derive(clap::Parser)]
pub struct Args {
    #[arg(short, long)]
    pub link: Vec<String>,

    pub input: String,
}
