mod config;
mod porkbun;

use crate::config::Config;
use crate::porkbun::client::PorkbunClient;
use clap::{Parser, Subcommand};
use eyre::Context;
use std::fs;

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

    match &cli.command {
        Commands::UpdateDomains { config } => update_domains(config),
        Commands::GetCertificates { config } => get_certificates(config),
    }
}

fn update_domains(config_path: &str) -> eyre::Result<()> {
    let config = Config::from_yaml(config_path)?;
    let client = PorkbunClient::new(config.api_key.into(), config.api_secret.into());
    let ping_response = client.ping()?;
    let records_response = client.retrieve_records(&config.domain, None)?;
    let domains = config
        .sub_domains
        .iter()
        .map(|domain| format!("{}.{}", domain, config.domain))
        .collect::<Vec<String>>();

    for record in records_response.records.iter().filter(|record| {
        record.record_type == "A" && domains.iter().any(|domain| domain == &record.name)
    }) {
        println!("Deleting record {}", record.name);
        client.delete_record_by_domain_and_id(&config.domain, &record.id)?;
    }

    for sub_domain in config.sub_domains {
        println!(
            "Creating record for {}.{} with ip: {}",
            &sub_domain, &config.domain, &ping_response.your_ip
        );
        client.create_record(
            &config.domain,
            &sub_domain,
            "A",
            &ping_response.your_ip,
            "600",
        )?;
    }

    Ok(())
}

fn get_certificates(config_path: &str) -> eyre::Result<()> {
    let config = Config::from_yaml(config_path)?;
    let client = PorkbunClient::new(config.api_key.into(), config.api_secret.into());
    let response = client.ssl_retrieve_bundle_by_domain(&config.domain)?;

    fs::write(
        format!("{}/{}", config.ssl_path, "certificate_chain.pub"),
        response.certificate_chain,
    )
    .context("write certificate_chain.pub")?;

    fs::write(
        format!("{}/{}", config.ssl_path, "intermediate_certificate.pub"),
        response.intermediate_certificate,
    )
    .context("write intermediate_certificate.pub")?;

    fs::write(
        format!("{}/{}", config.ssl_path, "private_key.pub"),
        response.private_key,
    )
    .context("write private_key.pub")?;

    fs::write(
        format!("{}/{}", config.ssl_path, "public_key.pub"),
        response.public_key,
    )
    .context("write certificate_chain.pub")?;

    Ok(())
}
