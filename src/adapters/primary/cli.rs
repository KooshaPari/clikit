//! CLI adapter for clikit - small example to host phenotype-cli-core parsing

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "clikit-example")]
pub struct Cli {
    #[arg(long)]
    pub config: Option<String>,
}

pub fn run_cli() {
    let cli = Cli::parse();
    println!("clikit cli parsed: config={:?}", cli.config);
}
