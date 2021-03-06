pub const MAKE_MOVE_TOPIC: &str = "bugout-make-move-cmd";
pub const PROVIDE_HISTORY_TOPIC: &str = "bugout-provide-history-cmd";
pub const JOIN_PRIVATE_GAME_TOPIC: &str = "bugout-join-private-game-cmd";
pub const FIND_PUBLIC_GAME_TOPIC: &str = "bugout-find-public-game-cmd";
pub const CREATE_GAME_TOPIC: &str = "bugout-create-game-cmd";
pub const CHOOSE_COLOR_PREF_TOPIC: &str = "bugout-choose-color-pref-cmd";
/// Will be used in participation service
pub const _QUIT_GAME_TOPIC: &str = "bugout-quit-game-cmd";
pub const ATTACH_BOT_TOPIC: &str = "bugout-attach-bot-cmd";
pub const REQ_SYNC_TOPIC: &str = "bugout-req-sync-cmd";
pub const UNDO_MOVE_TOPIC: &str = "bugout-undo-move-cmd";

/// A move was made and judged fit for communication to
/// all interested clients
pub const MOVE_MADE_TOPIC: &str = "bugout-move-made-ev";
pub const HISTORY_PROVIDED_TOPIC: &str = "bugout-history-provided-ev";
pub const PRIVATE_GAME_REJECTED_TOPIC: &str = "bugout-private-game-rejected-ev";
pub const GAME_READY_TOPIC: &str = "bugout-game-ready-ev";
pub const WAIT_FOR_OPPONENT_TOPIC: &str = "bugout-wait-for-opponent-ev";
pub const COLORS_CHOSEN_TOPIC: &str = "bugout-colors-chosen-ev";
pub const SESSION_DISCONNECTED_TOPIC: &str = "bugout-session-disconnected-ev";
pub const BOT_ATTACHED_TOPIC: &str = "bugout-bot-attached-ev";
pub const SYNC_REPLY_TOPIC: &str = "bugout-sync-reply-ev";
pub const MOVE_UNDONE_TOPIC: &str = "bugout-move-undone-ev";
pub const UNDO_REJECTED_TOPIC: &str = "bugout-undo-rejected-ev";
