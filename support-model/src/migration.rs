// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub async fn run_migrations(url: &str) -> Result<(), anyhow::Error> {
    run_migrations_on_conn(url).await?;
    Ok(())
}

pub async fn run_migrations_on_conn(url: &str) -> Result<(), anyhow::Error> {
    v_api_installer::run_migrations(url);
    Ok(())
}
