use clap::crate_version;

pub const CLIENT_ID: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    "/releases/tag/",
    crate_version!()
);
pub const BUFFER_SIZE: usize = 26_214_400;

pub const HTTP_HEADER_SERVER: &str = "server";
