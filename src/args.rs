use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ServerArgs {
    #[arg(short, long, default_value_t = String::from("frontman.toml"))]
    pub config_path: String,

    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}
