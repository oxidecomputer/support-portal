use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use strum::EnumIter;
use v_api::permissions::VPermission;
use v_api_permission_derive::v_api;

#[v_api(From(VPermission))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, JsonSchema, EnumIter)]
pub enum ApiPermissions {}
