mod config;

use crate::config::config::Config;
use clap::{Parser, Subcommand};
use porkbun_rs::{api, api::Query, auth::Auth, endpoints, Porkbun};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    UpdateDomains { config: String },
    GetCertificates { config: String },
}

fn main() -> eyre::Result<()> {
    let cli: Cli = Cli::parse();

    return match &cli.command {
        Commands::UpdateDomains { config } => update_domains(config),
        Commands::GetCertificates { config } => get_certificates(config),
    };

    let auth = Auth::new("apikey".into(), "apisecret".into());
    let client = Porkbun::new(auth)?;
    let endpoint = endpoints::Ping::builder().build()?;

    api::ignore(endpoint).query(&client)?;

    Ok(())
}

fn update_domains(config_path: &str) -> eyre::Result<()> {
    let config = Config::from_yaml(config_path)?;
    println!("{}", config.api_key);
    Ok(())
}

fn get_certificates(config_path: &str) -> eyre::Result<()> {
    let config = Config::from_yaml(config_path)?;
    println!("{}", config.api_key);
    Ok(())
}
