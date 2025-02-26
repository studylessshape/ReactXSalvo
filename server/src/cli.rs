use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct StartArgs {
    #[arg(short, long, default_value_t = false)]
    pub debug: bool
}