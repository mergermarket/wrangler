mod builder;
mod dev;
mod environment;
mod kv_namespace;
mod manifest;
mod route;
mod site;
mod target;
mod target_type;
mod triggers;

pub use builder::{Builder, ModuleRule, UploadFormat};
pub use environment::Environment;
pub use kv_namespace::{ConfigKvNamespace, KvNamespace};
pub use manifest::Manifest;
pub use route::{Route, RouteConfig};
pub use site::Site;
pub use target::Target;
pub use target_type::TargetType;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UsageModel {
    Bundled,
    Unbound,
}

impl FromStr for UsageModel {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bundled" => Ok(UsageModel::Bundled),
            "unbound" => Ok(UsageModel::Unbound),
            _ => failure::bail!("Invalid usage model; must be either \"bundled\" or \"unbound\""),
        }
    }
}

impl AsRef<str> for UsageModel {
    fn as_ref(&self) -> &str {
        match self {
            UsageModel::Bundled => "bundled",
            UsageModel::Unbound => "unbound",
        }
    }
}

#[cfg(test)]
mod tests;
