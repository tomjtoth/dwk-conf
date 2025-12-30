use std::{env, sync::LazyLock};

pub(crate) static IMAGE_PATH: LazyLock<String> =
    LazyLock::new(|| env::var("IMAGE_PATH").expect("missing env var IMAGE_PATH"));

pub(super) static IP: LazyLock<String> =
    LazyLock::new(|| env::var("IP").expect("missing env var IP"));

pub(super) static PORT: LazyLock<String> =
    LazyLock::new(|| env::var("PORT").expect("missing env var PORT"));

pub(crate) static CHANGE_INTERVAL: LazyLock<u64> = LazyLock::new(|| {
    env::var("CHANGE_INTERVAL")
        .expect("missing env var CHANGE_INTERVAL")
        .parse::<u64>()
        .expect("CHANGE_INTERVAL must be a valid u64")
});

pub(crate) static BACKEND_URL: LazyLock<String> =
    LazyLock::new(|| env::var("BACKEND_URL").expect("missing env var BACKEND_URL"));
