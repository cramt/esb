use crate::mc::get_status;
use crate::{
    constants::{mc_host, mc_port},
    test_helper::load_env,
};
use std::process::Stdio;
use std::{process::Command, thread, time::Duration};
use tokio_test::block_on;

use once_cell::sync::Lazy;
use speculate::speculate;

fn start_mc_server() {
    static S: Lazy<()> = Lazy::new(|| {
        if !String::from_utf8(
            Command::new("docker")
                .arg("ps")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .contains("itzg/minecraft-server")
        {
            Command::new("docker")
                .args(["compose", "up", "-d"])
                .current_dir("./integration_docker")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            thread::sleep(Duration::from_secs(30));
        }
    });
    let _ = *S;
}

speculate! {
    describe "mc_status" {
        before {
            load_env();
            start_mc_server();
        }

        after {

        }

        it "mc host is correct" {
            assert_eq!("localhost", mc_host());
        }

        it "mc port is correct" {
            assert_eq!(25565, mc_port());
        }

        it "should return the number of players" {
            let result = block_on(get_status()).unwrap();
            assert_eq!(result.players.online, 0)
        }
    }
}
