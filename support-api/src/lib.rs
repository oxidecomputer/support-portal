// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::BuildError as DropshotError;
use secrecy::ExposeSecret;
use slog::Logger;
use std::{
    path::PathBuf,
    process,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tokio::select;
use v_api::{
    VContextBuilder, VContextBuilderError,
    endpoints::login::oauth::{OAuthProviderName, remote::zendesk::ZendeskOAuthProvider},
};
use v_api_param::ParamResolutionError;

use crate::{
    context::{ApiContextBuilder, ApiContextBuilderError},
    initial_data::{InitError, InitialData},
    permissions::ApiPermissions,
};

pub mod config;
pub mod context;
mod initial_data;
pub mod permissions;
mod server;

pub use config::ServerConfig;
pub use server::{create_server, describe};

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Failed to build ApiContext")]
    CtxBuilder(#[from] ApiContextBuilderError),
    #[error("Failed to initialize data")]
    InitError(#[from] InitError),
    #[error("Failed to create server")]
    FailedToCreate(#[from] DropshotError),
    #[error("Failed to resolve paramater")]
    ParamResolution(#[from] ParamResolutionError),
    #[error("Failed to read measurement file")]
    Read(#[from] std::io::Error),
    #[error("Task failed")]
    TaskFailed(String),
    #[error("Failed to build VContext")]
    VBuilder(#[from] VContextBuilderError),
}

pub async fn run_server(
    config: ServerConfig,
    param_path: Option<PathBuf>,
    logger: Logger,
) -> Result<(), ServerError> {
    let database_url_secret = config.database_url.resolve(param_path.clone())?;

    let mut v_ctx = VContextBuilder::new()
        .with_public_url(config.public_url.clone())
        .with_jwt_expiration(config.jwt.default_expiration)
        .with_storage_url(database_url_secret.expose_secret().to_string())
        .with_keys(config.jwt.keys)
        .with_additional_builtin_permissions(vec![
            ApiPermissions::CreateOAuthClient,
            ApiPermissions::ManageOAuthClientsAll,
        ])
        .build()
        .await?;

    // Install OAuth provider
    if let Some(zendesk) = config.authn.oauth.zendesk {
        let resolved = zendesk.resolve(param_path)?;
        // let device_secret = zendesk
        //     .proxy_web
        //     .device
        //     .client_secret
        //     .resolve(config.param_base_path.clone())?;
        // let web_secret = zendesk
        //     .web
        //     .client_secret
        //     .resolve(config.param_base_path.clone())?;
        let public_url = config.public_url.clone();
        v_ctx.insert_oauth_provider(
            OAuthProviderName::Zendesk,
            Box::new(move || {
                let resolved = resolved.clone();
                Box::new(ZendeskOAuthProvider::new(
                    resolved,
                    public_url.clone(),
                    "oxidecomputerhelp".to_string(),
                    // zendesk.device.client_id.clone(),
                    // device_secret.clone(),
                    // zendesk.web.client_id.clone(),
                    // web_secret.clone(),
                    None,
                ))
            }),
        );

        tracing::info!("Added Zendesk OAuth provider");
    }

    let init_data = InitialData::new(config.initial_mappers.map(|p| vec![p])).map_err(|err| {
        tracing::error!(?err, "Failed to load initial data from configuration");
        err
    })?;
    init_data.initialize(&v_ctx).await.map_err(|err| {
        tracing::error!(?err, "Failed to install initial data");
        err
    })?;

    let ctx = ApiContextBuilder::default()
        .public_url(config.public_url.clone())
        .v_ctx(Arc::new(v_ctx))
        .build()?;

    set_ctrlc_handler(move || {
        tracing::info!("Received shutdown signal");
        0
    })
    .expect("Failed to install ctrl+c handler");

    let starter = create_server(ctx, logger, config.port.unwrap_or(8080)).build_starter()?;
    let server_task = starter.start();

    let error = select! {
        v = server_task => v.map_err(|e| ServerError::TaskFailed(e.to_string())),
    };

    error
}

pub fn set_ctrlc_handler<F>(f: F) -> Result<(), ctrlc::Error>
where
    F: FnOnce() -> i32 + Send + 'static,
{
    let f = Mutex::new(Some(f));
    ctrlc::set_handler(move || {
        if let Ok(mut guard) = f.lock() {
            let f = guard.take().expect("f can only be taken once");
            let code = f();
            process::exit(code);
        }
    })
}
