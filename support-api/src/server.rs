// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dropshot::{ApiDescription, ServerBuilder};
use slog::Logger;
use std::net::SocketAddr;

use crate::{context::ApiContext, permissions::ApiPermissions};

pub fn create_server(ctx: ApiContext, logger: Logger, port: u16) -> ServerBuilder<ApiContext> {
    let description = describe();

    let server =
        dropshot::ServerBuilder::new(description, ctx, logger).config(dropshot::ConfigDropshot {
            default_request_body_max_bytes: 10 * 1024 * 1024,
            bind_address: SocketAddr::from(([0, 0, 0, 0], port)),
            ..Default::default()
        });
    server
}

v_api::v_system_endpoints!(ApiContext, ApiPermissions);

pub fn describe() -> ApiDescription<ApiContext> {
    let mut description = ApiDescription::new();

    v_api::inject_endpoints!(description);

    description
}
