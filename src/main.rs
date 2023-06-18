mod config;
mod porkbun;

use crate::config::Config;
use crate::porkbun::client::PorkbunClient;
use crate::porkbun::responses::DnsRecord;
use clap::{Parser, Subcommand};
use eyre::Context;
use openssl::asn1::Asn1Time;
use openssl::x509::X509;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

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
    MonitorCertificate { config: String },
}

const DEFAULT_CERTIFICATE_CHAIN: &str = "domain.cert.pem";
const DEFAULT_INTERMEDIATE_CERTIFICATE: &str = "intermediate.cert.pem";
const DEFAULT_PRIVATE_KEY: &str = "private.key.pem";
const DEFAULT_PUBLIC_KEY: &str = "public.key.pem";

fn main() -> eyre::Result<()> {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::UpdateDomains { config } => update_domains(config),
        Commands::GetCertificates { config } => get_certificates(config),
        Commands::MonitorCertificate { config } => check_certificate(config),
    }
}

fn update_domains(config_path: &str) -> eyre::Result<()> {
    let config = Config::from_yaml(config_path)?;
    let client = PorkbunClient::new(config.api_key.into(), config.api_secret.into());

    println!("Getting device ip...");
    let my_ip = client.ping()?.your_ip;
    println!("Device ip: {}", &my_ip);

    let a_dns_domain_records = client
        .retrieve_records(&config.domain, None)?
        .records
        .drain(0..)
        .filter(|record| record.record_type == "A")
        .collect::<Vec<DnsRecord>>();

    let config_domains_sub_domains = config
        .sub_domains
        .iter()
        .map(|sub_domain| {
            (
                format!("{}.{}", &sub_domain, config.domain),
                sub_domain.to_owned(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let records_to_create = config_domains_sub_domains.iter().filter(|(domain, _)| {
        !a_dns_domain_records
            .iter()
            .any(|dns_record| dns_record.name.eq(domain))
    });

    let records_to_delete = a_dns_domain_records.iter().filter(|dns_record| {
        config_domains_sub_domains
            .iter()
            .any(|(domain, _)| dns_record.name.eq(domain))
    });

    let records_to_update = a_dns_domain_records.iter().filter(|dns_record| {
        dns_record.content != my_ip
            && !records_to_delete
                .clone()
                .any(|record| dns_record.id.eq(&record.id))
    });

    // Create the new domains
    for (domain, sub_domain) in records_to_create {
        println!("Creating new A record entry {}", domain);
        client.create_record(&config.domain, sub_domain, "A", &my_ip, "600")?;
    }

    // Update records
    for record_to_update in records_to_update {
        println!("Updating record {}", &record_to_update.name);
        client.edit_record_by_domain_and_id(
            &config.domain,
            &record_to_update.id,
            &record_to_update.name,
            &record_to_update.record_type,
            &my_ip,
        )?;
    }

    // Delete the obsolete records
    for dns_records in records_to_delete {
        println!("Deleting record with id {}", &dns_records.id);
        client.delete_record_by_domain_and_id(&config.domain, &dns_records.id)?;
    }

    Ok(())
}

fn get_certificates(config_path: &str) -> eyre::Result<()> {
    let config = Config::from_yaml(config_path)?;
    let client = PorkbunClient::new(config.api_key.into(), config.api_secret.into());
    let response = client.ssl_retrieve_bundle_by_domain(&config.domain)?;

    fs::write(
        format!(
            "{}/{}",
            &config.ssl.path,
            config
                .ssl
                .certificate_chain
                .unwrap_or_else(|| DEFAULT_CERTIFICATE_CHAIN.to_string())
        ),
        response.certificate_chain,
    )
    .context("write certificate_chain.pub")?;

    fs::write(
        format!(
            "{}/{}",
            &config.ssl.path,
            config
                .ssl
                .intermediate_certificate
                .unwrap_or_else(|| DEFAULT_INTERMEDIATE_CERTIFICATE.to_string())
        ),
        response.intermediate_certificate,
    )
    .context("write intermediate_certificate.pub")?;

    fs::write(
        format!(
            "{}/{}",
            &config.ssl.path,
            config
                .ssl
                .private_key
                .unwrap_or_else(|| DEFAULT_PRIVATE_KEY.to_string())
        ),
        response.private_key,
    )
    .context("write private_key.pub")?;

    fs::write(
        format!(
            "{}/{}",
            &config.ssl.path,
            config
                .ssl
                .public_key
                .unwrap_or_else(|| DEFAULT_PUBLIC_KEY.to_string())
        ),
        response.public_key,
    )
    .context("write certificate_chain.pub")?;

    Ok(())
}

fn check_certificate(config_path: &str) -> eyre::Result<()> {
    let config = Config::from_yaml(config_path)?;
    let ssl_cert_path = format!(
        "{}/{}",
        &config.ssl.path,
        config
            .ssl
            .certificate_chain
            .unwrap_or_else(|| DEFAULT_CERTIFICATE_CHAIN.to_string())
    );

    loop {
        let ssl_cert_bytes = fs::read(&ssl_cert_path).context("reading ssl certificate")?;
        let certificate = X509::from_pem(ssl_cert_bytes.as_slice())?;
        let certificate_expiration = certificate.not_after();
        let threshold = Asn1Time::days_from_now(0).unwrap();
        let time_dif = threshold.diff(&certificate_expiration)?;

        if time_dif.days < 10 {
            let result = get_certificates(config_path);
            if result.is_err() {
                println!(
                    "failed to update the certificates {}",
                    result.unwrap_err().to_string()
                );
                sleep(Duration::from_secs(30))
            }
        } else {
            println!(
                "certificate expires on: {}",
                certificate_expiration.to_string()
            );
            let sleep_duration = Duration::from_secs(
                u64::try_from(time_dif.days - 10).context(format!(
                    "converting i32 to u64, value: {}",
                    time_dif.days - 20
                ))? * 24
                    * 3600,
            );
            println!("next update in: {} seconds", sleep_duration.as_secs());
            sleep(sleep_duration);
        }
    }
}
