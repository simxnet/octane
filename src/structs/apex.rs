use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApexPlayer {
    pub global: Global,
    pub realtime: Realtime,
    pub legends: Legends,
    #[serde(rename = "mozambiquehere_internal")]
    pub mozambiquehere_internal: MozambiquehereInternal,
    pub total: Total,
    pub processing_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Global {
    pub name: String,
    pub tag: String,
    pub uid: String,
    pub avatar: String,
    pub platform: String,
    pub level: i64,
    pub to_next_level_percent: i64,
    pub internal_update_count: i64,
    pub bans: Bans,
    pub rank: Rank,
    pub arena: Arena,
    pub battlepass: Battlepass,
    pub internal_parsing_version: i64,
    pub badges: Vec<Badge>,
    pub level_prestige: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bans {
    pub is_active: bool,
    pub remaining_seconds: i64,
    #[serde(rename = "last_banReason")]
    pub last_ban_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub rank_score: i64,
    pub rank_name: String,
    pub rank_div: i64,
    pub ladder_pos_platform: i64,
    pub rank_img: String,
    pub ranked_season: String,
    #[serde(rename = "ALStopPercent")]
    pub alstop_percent: i64,
    #[serde(rename = "ALStopInt")]
    pub alstop_int: i64,
    #[serde(rename = "ALStopPercentGlobal")]
    pub alstop_percent_global: i64,
    #[serde(rename = "ALStopIntGlobal")]
    pub alstop_int_global: i64,
    #[serde(rename = "ALSFlag")]
    pub alsflag: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arena {
    pub rank_score: i64,
    pub rank_name: String,
    pub rank_div: i64,
    pub ladder_pos_platform: i64,
    pub rank_img: String,
    pub ranked_season: String,
    #[serde(rename = "ALStopPercent")]
    pub alstop_percent: String,
    #[serde(rename = "ALStopInt")]
    pub alstop_int: String,
    #[serde(rename = "ALStopPercentGlobal")]
    pub alstop_percent_global: String,
    #[serde(rename = "ALStopIntGlobal")]
    pub alstop_int_global: String,
    #[serde(rename = "ALSFlag")]
    pub alsflag: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battlepass {
    pub level: Value,
    pub history: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub name: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Realtime {
    pub lobby_state: String,
    pub is_online: i64,
    pub is_in_game: i64,
    pub can_join: i64,
    pub party_full: i64,
    pub selected_legend: String,
    pub current_state: String,
    pub current_state_since_timestamp: i64,
    pub current_state_as_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legends {
    pub selected: Selected,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selected {
    #[serde(rename = "LegendName")]
    pub legend_name: String,
    pub data: Vec<Value>,
    pub game_info: GameInfo,
    #[serde(rename = "ImgAssets")]
    pub img_assets: ImgAssets,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub skin: String,
    pub skin_rarity: String,
    pub frame: String,
    pub frame_rarity: String,
    pub pose: String,
    pub pose_rarity: String,
    pub intro: String,
    pub intro_rarity: String,
    pub badges: Vec<Badge2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge2 {
    pub name: String,
    pub value: i64,
    pub category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImgAssets {
    pub icon: String,
    pub banner: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MozambiquehereInternal {
    #[serde(rename = "isNewToDB")]
    pub is_new_to_db: bool,
    pub cluster_srv: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Total {
    pub kd: Kd,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kd {
    pub value: String,
    pub name: String,
}

