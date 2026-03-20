#![allow(dead_code)]

mod artifact;
pub(crate) mod cfg;
pub(crate) mod compile;
pub(crate) mod ir;
pub(crate) mod ssa;
mod runtime;

pub use artifact::AotArtifactError;
pub(crate) use compile::CompiledProgram;
