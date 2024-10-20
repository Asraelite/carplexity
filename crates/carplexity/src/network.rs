// Idea:
// For the game server sending game state to clients:
// For a given player/car, predict the next user input by cubically interpolating between the last few inputs that are known.
// When the actual input diverges too much from the predicted input, send a packet with the new input.
// Gradually reduce the tolerance for divergence as more time passes without sending a packet. Reset the tolerance when a packet is sent.

use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::vehicle::control::{VehicleControlAction, VehicleControlInputState};

pub const DEFAULT_IDENTITY_SERVER_ADDR: &str = "localhost";
pub const DEFAULT_IDENTITY_SERVER_PORT: u16 = 13705;
pub const DEFAULT_GAME_SERVER_PORT: u16 = 13706;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
	fn build(&self, app: &mut App) {
		// app.add_systems(Update, update_network);
	}
}

#[derive(Component)]
pub struct IdentityServer;

#[derive(Component)]
pub struct GameServer;

#[derive(Debug, Component)]
pub struct RemoteConnection {
	pub ip: IpAddr,
	pub port: u16,
}

#[derive(Debug, Component)]
pub enum ConnectionState {
	Unconnected,
	Connecting,
	Connected,
}

#[derive(Debug, Component)]
pub struct PingEstimate {
	pub tick: u64,
	pub ping: f32,
}

#[derive(Bundle)]
pub struct RemoteServerBundle {
	pub connection: RemoteConnection,
	pub ping_estimate: PingEstimate,
	pub connection_state: ConnectionState,
}

impl RemoteServerBundle {
	pub fn new(ip: IpAddr, port: u16) -> Self {
		Self {
			connection: RemoteConnection { ip, port },
			ping_estimate: PingEstimate { tick: 0, ping: 0.0 },
			connection_state: ConnectionState::Unconnected,
		}
	}
}

#[derive(Debug)]
pub enum Source {
	Server(RemoteConnection),
	P2p, // TODO
}

#[derive(Debug)]
pub struct Packet {
	source: Source,
	tick: u64,
	data: PacketData,
}

#[derive(Debug)]
pub enum PacketData {
	SetVehicleControlInput(VehicleControlInputState),
	Action(Action),
}

#[derive(Debug)]
pub enum Action {
	SetVehicleControlInput(VehicleControlInputState),
	PerformVehicleControlAction(VehicleControlAction),
}

// fn update_network(mut packets: EventReader<Packet>) {
// 	for packet in packets.read() {
// 		println!("{:?}", packet);
// 	}
// }
