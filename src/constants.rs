use std::{collections::HashSet, env, ops::Deref};

use once_cell::sync::Lazy;
use shiplift::Docker;

pub fn application_id() -> u64 {
    static ENV: Lazy<u64> = Lazy::new(|| env::var("APPLICATION_ID").unwrap().parse().unwrap());
    *ENV
}

pub fn discord_secret() -> &'static str {
    static ENV: Lazy<String> = Lazy::new(|| env::var("DISCORD_SERCRET").unwrap());
    ENV.deref()
}

pub fn mc_uri() -> &'static str {
    static ENV: Lazy<String> = Lazy::new(|| env::var("MC_SERVER").unwrap());
    ENV.deref()
}

pub fn mc_host() -> &'static str {
    let uri = mc_uri();
    let pos = uri.chars().position(|x| x == ':').unwrap();
    &uri[0..pos]
}

pub fn mc_port() -> u16 {
    let uri = mc_uri();
    let pos = uri.chars().position(|x| x == ':').unwrap() + 1;
    uri[pos..uri.len()].parse().unwrap()
}

pub fn users() -> &'static HashSet<u64> {
    static ENV: Lazy<HashSet<u64>> = Lazy::new(|| {
        env::var("USERS")
            .unwrap()
            .split('|')
            .map(|id| id.parse().unwrap())
            .collect()
    });
    ENV.deref()
}

pub fn modded_mc_container_name() -> &'static str {
    static ENV: Lazy<String> = Lazy::new(|| env::var("MODDED_MC_CONTAINER_NAME").unwrap());
    ENV.deref()
}

pub fn docker_socket() -> &'static str {
    static ENV: Lazy<String> = Lazy::new(|| env::var("DOCKER_SOCKET").unwrap());
    ENV.deref()
}

pub fn docker_conn() -> &'static Docker {
    static CONN: Lazy<Docker> = Lazy::new(|| Docker::unix(docker_socket()));
    CONN.deref()
}
