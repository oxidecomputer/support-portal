// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use dropshot::semver::Version;
use rustls::crypto;
use secrecy::ExposeSecret;
use slog::Drain;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use support_api::{ServerConfig, describe};
use tap::TapFallible;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_slog::TracingSlogDrain;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use v_api::config::{ServerLogFormat, SpecConfig};

static TITLE: &str = "Customer Support API";
static DESCRIPTION: &str = "Programmatic access to the Oxide customer support API.";
static CONTACT_URL: &str = "https://oxide.computer";
static CONTACT_EMAIL: &str = "corp-services@oxidecomputer.com";
static OUTPUT_PATH: &str = "support-api-spec.json";

#[derive(Parser, Debug)]
pub struct ServerCli {
    #[clap(long, short)]
    pub config: Option<String>,
    #[clap(subcommand)]
    pub command: ServerCommand,
}

#[derive(Parser, Debug, Clone)]
pub enum ServerCommand {
    /// Regenerate the OpenAPI schema file
    Describe,
    /// Apply database migrations
    Migrate,
    /// Run the api server
    Run,
    /// Validate the currently located configuration file
    Validate,
    /// Print the version
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crypto::ring::default_provider()
        .install_default()
        .expect("Installed default crypto provider");

    let args = ServerCli::parse();

    match args.command {
        ServerCommand::Describe => {
            describe_server(SpecConfig {
                title: TITLE.to_string(),
                description: DESCRIPTION.to_string(),
                contact_url: CONTACT_URL.to_string(),
                contact_email: CONTACT_EMAIL.to_string(),
                output_path: PathBuf::from(OUTPUT_PATH),
            })?;
        }
        ServerCommand::Migrate => {
            let config = ServerConfig::new(args.config.map(|path| vec![path]))?;
            support_model::migration::run_migrations(
                config
                    .database_url
                    .resolve(config.param_base_path)?
                    .expose_secret(),
            )
            .await?;
            println!("Migrations completed successfully");
        }
        ServerCommand::Run => {
            let node_id = Uuid::new_v4();
            let config = ServerConfig::new(args.config.map(|path| vec![path]))?;
            run_server(config, node_id).await?;
        }
        ServerCommand::Validate => {
            let config = ServerConfig::new(args.config.map(|path| vec![path]));
            match config {
                Ok(_) => println!("Loaded settings successfully"),
                Err(err) => eprintln!("Failed to load settings file:\n{}", err),
            }
        }
        ServerCommand::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

pub fn register_logger(
    log_format: &ServerLogFormat,
    log_directory: Option<&Path>,
) -> anyhow::Result<WorkerGuard> {
    let (writer, guard) = if let Some(log_directory) = log_directory {
        let file_appender = tracing_appender::rolling::daily(log_directory, "support-portal.log");
        tracing_appender::non_blocking(file_appender)
    } else {
        NonBlocking::new(std::io::stdout())
    };

    let subscriber = tracing_subscriber::fmt()
        .with_file(false)
        .with_line_number(false)
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(writer);

    match log_format {
        ServerLogFormat::Json => subscriber.json().init(),
        ServerLogFormat::Pretty => subscriber.pretty().init(),
    };

    tracing::info!("Initialized logger");

    Ok(guard)
}

fn describe_server(config: SpecConfig) -> Result<(), dropshot::semver::Error> {
    let api = describe();
    // Create the API schema.
    let mut api_definition =
        &mut api.openapi(config.title, Version::parse(env!("CARGO_PKG_VERSION"))?);
    api_definition = api_definition
        .description(config.description)
        .contact_url(config.contact_url)
        .contact_email(config.contact_email);

    let mut buffer = File::create(&config.output_path).unwrap();
    api_definition.write(&mut buffer).unwrap();
    println!(
        "API spec written to {}",
        config.output_path.to_string_lossy()
    );

    Ok(())
}

async fn run_server(config: ServerConfig, node_id: Uuid) -> anyhow::Result<()> {
    let params_base_path = std::env::var("PARAMS_BASE_PATH").map(PathBuf::from).ok();

    let _log_guard = register_logger(&config.log_format, config.log_directory.as_deref())?;

    // Construct a shim to pipe slog logs into the global tracing logger
    let slog_logger = {
        let level_drain = slog::LevelFilter(TracingSlogDrain, slog::Level::Debug).fuse();
        let async_drain = slog_async::Async::new(level_drain).build().fuse();
        slog::Logger::root(async_drain, slog::o!("node" => node_id.to_string()))
    };

    support_api::run_server(config, params_base_path, slog_logger)
        .await
        .tap_err(|err| {
            tracing::error!(?err, "Server exited unexpectedly");
        })?;

    drop(_log_guard);

    Ok(())
}
