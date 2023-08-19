//! Domains is called by all modules.
#![forbid(unsafe_code)]

use sqlx::{Pool, Sqlite};

pub mod posts;

#[macro_use]
extern crate tracing;

pub struct DBPool(pub Pool<Sqlite>);
