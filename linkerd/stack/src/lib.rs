//! Utiliteis for composing

#![deny(warnings, rust_2018_idioms)]

pub mod fallback;
mod future_service;
pub mod layer;
pub mod map_response;
pub mod map_target;
pub mod new_service;
pub mod on_response;
pub mod oneshot;
pub mod pending;
pub mod proxy;

pub use self::fallback::{Fallback, FallbackLayer};
pub use self::future_service::FutureService;
pub use self::map_target::{MapTarget, MapTargetLayer, MapTargetService};
pub use self::on_response::{OnResponse, OnResponseLayer};
pub use self::{new_service::NewService, proxy::Proxy};
