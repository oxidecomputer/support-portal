// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use derive_builder::Builder;
use v_api::{ApiContext as VApiContext, VContext};

use crate::permissions::ApiPermissions;

#[derive(Builder, Clone)]
#[builder(pattern = "owned")]
pub struct ApiContext {
    pub public_url: String,
    v_ctx: Arc<VContext<ApiPermissions>>,
}

impl VApiContext for ApiContext {
    type AppPermissions = ApiPermissions;
    fn v_ctx(&self) -> &VContext<Self::AppPermissions> {
        &self.v_ctx
    }
}
