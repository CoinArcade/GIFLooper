pub mod from;
mod into_shared;

pub use into_shared::*;

use crate::model::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JoinPrivateGameBackendCommand {
    #[serde(rename = "gameId")]
    pub game_id: GameId,
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindPublicGameBackendCommand {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChooseColorPrefBackendCommand {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "colorPref")]
    pub color_pref: ColorPref,
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}

/// Gateway may manually create private games,
/// but it will never create a public game.
/// We omit specifying the game ID here, and
/// let game lobby choose it for us.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGameBackendCommand {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    pub visibility: Visibility,
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
    #[serde(rename = "boardSize")]
    pub board_size: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum HeartbeatType {
    WebSocketPong,
    UserInterfaceBeep,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientHeartbeat {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "heartbeatType")]
    pub heartbeat_type: HeartbeatType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionDisconnected {
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuitGameCommand {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "gameId")]
    pub game_id: GameId,
}

/// This command is intended to be paired with SessionId
/// when it arrives at its redis stream.  SessionId is
/// generated by gateway and not subject to malicious
/// hoodwinking by javascript genies.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReqSyncBackendCommand {
    pub session_id: SessionId,
    pub req_id: ReqId,
    pub player_up: Player,
    pub turn: u32,
    pub last_move: Option<Move>,
    pub game_id: GameId,
}

/// Inputs to one of the backends
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BackendCommands {
    MakeMove(MakeMoveCommand),
    ProvideHistory(ProvideHistoryCommand),
    JoinPrivateGame(JoinPrivateGameBackendCommand),
    FindPublicGame(FindPublicGameBackendCommand),
    CreateGame(CreateGameBackendCommand),
    ChooseColorPref(ChooseColorPrefBackendCommand),
    ClientHeartbeat(ClientHeartbeat),
    SessionDisconnected(SessionDisconnected),
    QuitGame(QuitGameCommand),
    AttachBot(bot_model::api::AttachBot),
    ReqSync(ReqSyncBackendCommand),
    UndoMove(undo_model::api::UndoMove),
}
