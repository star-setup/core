pub mod resolved;
pub use resolved::{ResolvedArgs, ResolvedBuildFlags, ResolvedConnectionFlags, ResolvedMonoFlags};
pub mod resolver;
pub use resolver::{resolve_bool, resolve_with_config};
