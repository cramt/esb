#[cfg(test)]
mod test;

use std::io;

use craftio_rs::{
    CraftAsyncReader, CraftAsyncWriter, CraftConnection, CraftIo, CraftTokioConnection, ReadError,
    WriteError,
};
use mcproto_rs::{
    protocol::State,
    status::StatusSpec,
    types::VarInt,
    v1_15_2::{HandshakeNextState, HandshakeSpec, Packet578, RawPacket578, StatusRequestSpec},
    Deserialize,
};
use tokio::{
    io::BufReader,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

use crate::constants::{docker_conn, mc_host, mc_port, mc_uri, modded_mc_container_name};

#[derive(Debug)]
pub enum McError {
    TransportError(io::Error),
    McWriteError(WriteError),
    McReadError(ReadError),
    NotFound,
    NotRunning,
}

type McConnection = CraftConnection<BufReader<OwnedReadHalf>, OwnedWriteHalf>;

async fn create_connection() -> Result<McConnection, McError> {
    let mut conn = CraftTokioConnection::connect_server_tokio(mc_uri())
        .await
        .map_err(McError::TransportError)?;
    conn.write_packet_async(Packet578::Handshake(HandshakeSpec {
        version: VarInt::mc_deserialize(&[1, 12, 2]).unwrap().value,
        server_address: mc_host().to_string(),
        server_port: mc_port(),
        next_state: HandshakeNextState::Status,
    }))
    .await
    .map_err(McError::McWriteError)?;
    conn.set_state(State::Status);
    Ok(conn)
}
async fn running() -> bool {
    async fn inner() -> Option<bool> {
        let conn = docker_conn();
        let containers = conn.containers().list(&Default::default()).await.ok()?;
        let modded_mc_container = containers.into_iter().find(|container| {
            container
                .names
                .contains(&format!("/{}", modded_mc_container_name()))
        });
        Some(modded_mc_container.is_some())
    }
    inner().await.unwrap_or(false)
}

pub async fn get_status() -> Result<StatusSpec, McError> {
    if !running().await {
        return Err(McError::NotRunning);
    }
    let mut conn = create_connection().await?;
    conn.write_packet_async(Packet578::StatusRequest(StatusRequestSpec))
        .await
        .map_err(McError::McWriteError)?;
    let result = conn
        .read_packet_async::<RawPacket578>()
        .await
        .map_err(McError::McReadError)?
        .ok_or(McError::NotFound)?;
    match result {
        Packet578::StatusResponse(x) => Ok(x.response),
        _ => Err(McError::NotFound),
    }
}
