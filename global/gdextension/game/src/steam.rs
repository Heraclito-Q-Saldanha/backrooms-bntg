use godot::prelude::*;
use std::sync;
use std::time;

const CALLBACK_TIMEOUT: time::Duration = time::Duration::from_secs(32);

#[derive(GodotClass)]
#[class(base=Node, singleton)]
pub struct Steam {
	base: Base<Node>,
	client: steamworks::Client,
}

#[godot_api]
impl INode for Steam {
	fn init(base: Base<Node>) -> Self {
		let client = steamworks::Client::init_app(480).expect("fail to initialize steam");

		Self { base, client }
	}
	fn process(&mut self, _delta: f64) {
		self.client.run_callbacks();
	}
}

impl Steam {
	pub fn create_lobby(&self, lobby_type: steamworks::LobbyType, max_members: u32) -> steamworks::SResult<steamworks::LobbyId> {
		let (receiver, callback) = get_callback();
		self.client.matchmaking().create_lobby(lobby_type, max_members, callback);

		receiver.recv_timeout(CALLBACK_TIMEOUT).unwrap()
	}
	pub fn join_lobby(&self, lobby: steamworks::LobbyId) -> Result<steamworks::LobbyId, ()> {
		let (receiver, callback) = get_callback();
		self.client.matchmaking().join_lobby(lobby, callback);

		receiver.recv_timeout(CALLBACK_TIMEOUT).unwrap()
	}
	pub fn request_lobby_list(&self) -> steamworks::SResult<Vec<steamworks::LobbyId>> {
		let (receiver, callback) = get_callback();
		self.client.matchmaking().request_lobby_list(callback);

		receiver.recv_timeout(CALLBACK_TIMEOUT).unwrap()
	}
	pub fn leave_lobby(&self, lobby: steamworks::LobbyId) {
		self.client.matchmaking().leave_lobby(lobby);
	}
	pub fn lobby_members(&self, lobby: steamworks::LobbyId) -> Vec<steamworks::SteamId> {
		self.client.matchmaking().lobby_members(lobby)
	}
	pub fn set_lobby_joinable(&self, lobby: steamworks::LobbyId, joinable: bool) -> bool {
		self.client.matchmaking().set_lobby_joinable(lobby, joinable)
	}
	pub fn receive_messages_on_channel(&self, channel: u32, batch_size: usize) -> Vec<steamworks::networking_types::NetworkingMessage> {
		self.client.networking_messages().receive_messages_on_channel(channel, batch_size)
	}
	pub fn get_friend(&self, friend: steamworks::SteamId) -> steamworks::Friend {
		self.client.friends().get_friend(friend)
	}
	pub fn send_message_to_user(&self, user: steamworks::networking_types::NetworkingIdentity, send_type: steamworks::networking_types::SendFlags, data: &[u8], channel: u32) -> Result<(), steamworks::SteamError> {
		self.client.networking_messages().send_message_to_user(user, send_type, data, channel)
	}
}

fn get_callback<Args: Send + 'static>() -> (sync::mpsc::Receiver<Args>, impl Fn(Args)) {
	let (sender, receiver) = sync::mpsc::channel();

	let func = move |args: Args| {
		let _ = sender.send(args);
	};

	(receiver, func)
}
