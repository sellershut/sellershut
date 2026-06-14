use anyhow::{Context, Result};
use serde_json::{Value, json};
use tracing::{debug, error, info};
use vaultrs::{client::VaultClient, kv2, sys, token};

pub async fn check_vault_startup(client: &VaultClient, mount: &str) -> Result<()> {
    info!("checking vault status");

    let status = sys::status(client).await?;
    debug!(?status, "vault status received");

    info!("checking vault token");

    token::lookup_self(client)
        .await
        .context("Vault token validation failed")?;

    info!("vault token is valid");

    let healthcheck_path = "__healthcheck__/startup";

    info!(
        mount = %mount,
        path = %healthcheck_path,
        "checking kv-v2 write access"
    );

    kv2::set(
        client,
        mount,
        healthcheck_path,
        &json!({
            "status": "ok"
        }),
    )
    .await
    .inspect_err(|err| {
        error!(
            mount = %mount,
            path = %healthcheck_path,
            error = %err,
            "vault kv-v2 write check failed"
        );
    })
    .with_context(|| {
        format!("Vault KV write check failed. Is KV v2 enabled at mount '{mount}'?")
    })?;

    info!(
        mount = %mount,
        path = %healthcheck_path,
        "checking kv-v2 read access"
    );

    let _: Value = kv2::read(client, mount, healthcheck_path)
        .await
        .inspect_err(|err| {
            error!(
                mount = %mount,
                path = %healthcheck_path,
                error = %err,
                "vault kv-v2 read check failed"
            );
        })
        .with_context(|| {
            format!("Vault KV read check failed. Is KV v2 enabled at mount '{mount}'?")
        })?;

    info!("vault startup checks passed");

    Ok(())
}
