use godot::prelude::*;
use std::sync;

#[derive(GodotClass)]
#[class(base=Node, singleton)]
pub struct Steam {
	base: Base<Node>,
	client: steamworks::Client,
	callback_sender: sync::mpsc::Sender<CallbackValue>,
	callback_receiver: sync::mpsc::Receiver<CallbackValue>,
}

#[derive(GodotConvert, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[godot(transparent)]
pub struct LobbyId(i64);

#[derive(GodotConvert, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[godot(transparent)]
pub struct SteamId(i64);

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[godot(via = GString)]
#[repr(u8)]
pub enum LobbyType {
	Private = 0,
	FriendsOnly = 1,
	Public = 2,
	Invisible = 3,
}

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[godot(via = GString)]
pub enum LobbyEnterResult {
	LobbyEnterResponseSuccess = 1,
	LobbyEnterResponseDoesntExist = 2,
	LobbyEnterResponseNotAllowed = 3,
	LobbyEnterResponseFull = 4,
	LobbyEnterResponseError = 5,
	LobbyEnterResponseBanned = 6,
	LobbyEnterResponseLimited = 7,
	LobbyEnterResponseClanDisabled = 8,
	LobbyEnterResponseCommunityBan = 9,
	LobbyEnterResponseMemberBlockedYou = 10,
	LobbyEnterResponseYouBlockedMember = 11,
	LobbyEnterResponseRateLimitExceeded = 15,
}

#[derive(GodotClass)]
pub struct NetworkingMessage {
	data: Vec<u8>,
	steam_id: SteamId,
}

enum CallbackValue {
	LobbyCreated(LobbyEnterResult, LobbyId),
}

#[godot_api]
impl IRefCounted for NetworkingMessage {
	fn init(_: Base<RefCounted>) -> Self {
		Self {
			data: Vec::new(),
			steam_id: SteamId::from_u64(0),
		}
	}
}

#[godot_api]
impl INode for Steam {
	fn init(base: Base<Node>) -> Self {
		let client = steamworks::Client::init_app(480).expect("fail to initialize steam");
		let (callback_sender, callback_receiver) = sync::mpsc::channel();

		Self {
			base,
			client,
			callback_receiver,
			callback_sender,
		}
	}

	fn process(&mut self, _delta: f64) {
		self.client.run_callbacks();

		if let Ok(callback) = self.callback_receiver.try_recv() {
			match callback {
				CallbackValue::LobbyCreated(result, lobby_id) => {
					self.signals().lobby_created().emit(result, lobby_id);
				}
			}
		}
	}
}

#[godot_api]
impl Steam {
	#[signal]
	pub fn lobby_created(result: LobbyEnterResult, lobby_id: LobbyId);

	#[func]
	pub fn create_lobby(&mut self, lobby_type: LobbyType, max_members: u32) {
		let matchmaking = self.client.matchmaking();
		let callback_sender = self.callback_sender.clone();
		let lobby_type = lobby_type.into();

		matchmaking.create_lobby(lobby_type, max_members, move |result| match result {
			Ok(id) => {
				let id = id.into();
				let _ = callback_sender.send(CallbackValue::LobbyCreated(LobbyEnterResult::LobbyEnterResponseSuccess, id));
			}
			Err(err) => {
				let id = LobbyId::from_u64(0);
				let err = err.try_into().unwrap();
				let _ = callback_sender.send(CallbackValue::LobbyCreated(err, id));
			}
		});
	}

	#[func]
	pub fn leave_lobby(&self, lobby_id: LobbyId) {
		let lobby_id = lobby_id.into();
		self.client.matchmaking().leave_lobby(lobby_id);
	}

	#[func]
	pub fn lobby_members(&self, lobby_id: LobbyId) -> Vec<SteamId> {
		let lobby_id = lobby_id.into();
		let members = self.client.matchmaking().lobby_members(lobby_id);
		members.into_iter().map(|member| member.into()).collect()
	}

	#[func]
	pub fn set_lobby_joinable(&self, lobby_id: LobbyId, joinable: bool) -> bool {
		let lobby_id = lobby_id.into();
		self.client.matchmaking().set_lobby_joinable(lobby_id, joinable)
	}

	#[func]
	pub fn receive_messages_on_channel(&self, channel: u32, batch_size: u32) -> Vec<Gd<NetworkingMessage>> {
		let messages = self.client.networking_messages().receive_messages_on_channel(channel, batch_size as usize);
		messages.into_iter().map(|msg| Gd::from_object(msg.into())).collect()
	}
}

impl LobbyId {
	#[inline(always)]
	pub fn from_u64(id: u64) -> Self {
		Self(id.cast_signed())
	}
	#[inline(always)]
	pub fn to_u64(self) -> u64 {
		self.0.cast_unsigned()
	}
}

impl From<steamworks::LobbyId> for LobbyId {
	#[inline(always)]
	fn from(value: steamworks::LobbyId) -> Self {
		Self::from_u64(value.raw())
	}
}

impl From<LobbyId> for steamworks::LobbyId {
	#[inline(always)]
	fn from(value: LobbyId) -> Self {
		Self::from_raw(value.to_u64())
	}
}

impl SteamId {
	#[inline(always)]
	pub fn from_u64(id: u64) -> Self {
		Self(id.cast_signed())
	}
	#[inline(always)]
	pub fn to_u64(self) -> u64 {
		self.0.cast_unsigned()
	}
}

#[godot_api]
impl NetworkingMessage {
	#[func]
	pub fn data(&self) -> Vec<u8> {
		self.data.to_vec()
	}
	#[func]
	pub fn steam_id(&self) -> SteamId {
		self.steam_id
	}
}

impl From<steamworks::SteamId> for SteamId {
	#[inline(always)]
	fn from(value: steamworks::SteamId) -> Self {
		Self::from_u64(value.raw())
	}
}

impl From<SteamId> for steamworks::SteamId {
	#[inline(always)]
	fn from(value: SteamId) -> Self {
		Self::from_raw(value.to_u64())
	}
}

impl From<steamworks::LobbyType> for LobbyType {
	#[inline(always)]
	fn from(value: steamworks::LobbyType) -> Self {
		match value {
			steamworks::LobbyType::FriendsOnly => Self::FriendsOnly,
			steamworks::LobbyType::Invisible => Self::Invisible,
			steamworks::LobbyType::Private => Self::Private,
			steamworks::LobbyType::Public => Self::Private,
		}
	}
}

impl From<LobbyType> for steamworks::LobbyType {
	#[inline(always)]
	fn from(value: LobbyType) -> Self {
		match value {
			LobbyType::FriendsOnly => Self::FriendsOnly,
			LobbyType::Invisible => Self::Invisible,
			LobbyType::Private => Self::Private,
			LobbyType::Public => Self::Private,
		}
	}
}

impl From<steamworks::networking_types::NetworkingMessage> for NetworkingMessage {
	fn from(value: steamworks::networking_types::NetworkingMessage) -> Self {
		let data = value.data().to_vec();
		let steam_id = value.identity_peer().steam_id().map(|id| id.into()).unwrap_or(SteamId::from_u64(0));
		Self { data, steam_id }
	}
}

impl TryFrom<steamworks::SteamError> for LobbyEnterResult {
	type Error = steamworks::SteamError;

	fn try_from(value: steamworks::SteamError) -> Result<Self, Self::Error> {
		match value {
			steamworks::SteamError::NoMatch => Ok(Self::LobbyEnterResponseDoesntExist),
			steamworks::SteamError::AccessDenied => Ok(Self::LobbyEnterResponseNotAllowed),
			steamworks::SteamError::LimitExceeded => Ok(Self::LobbyEnterResponseFull),
			steamworks::SteamError::Banned => Ok(Self::LobbyEnterResponseBanned),
			steamworks::SteamError::LimitedUserAccount => Ok(Self::LobbyEnterResponseLimited),
			steamworks::SteamError::AccountDisabled => Ok(Self::LobbyEnterResponseClanDisabled),
			steamworks::SteamError::CommunityCooldown => Ok(Self::LobbyEnterResponseCommunityBan),
			steamworks::SteamError::Blocked => Ok(Self::LobbyEnterResponseMemberBlockedYou),
			steamworks::SteamError::Ignored => Ok(Self::LobbyEnterResponseYouBlockedMember),
			steamworks::SteamError::RateLimitExceeded => Ok(Self::LobbyEnterResponseRateLimitExceeded),
			err => Err(err),
		}
	}
}
