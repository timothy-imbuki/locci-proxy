use clap::Parser;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long, value_name = "FILE", env = "LOCCI_CONFIG")]
    pub config: Option<PathBuf>,
    #[arg(long, value_parser = ["load_balancer", "api_gateway"])]
    pub mode: Option<String>,
    #[arg(short, long)]
    pub bind: Option<String>,
    #[arg(short, long)]
    pub workers: Option<usize>,
}
