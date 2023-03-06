use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub async fn get_last_two_minutes(game_id: &str) -> Result<LastTwoMinutesReport, String> {
    get_json_data(&format!("https://official.nba.com/l2m/json/{game_id}.json")).await
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LastTwoMinutesReport {
    pub game: Vec<LastTwoMinutesGame>,
    pub stats: Vec<LastTwoMinutesStat>,
    pub l2m: Vec<LastTwoMinutesEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastTwoMinutesGame {
    #[serde(rename = "Home_team", default)]
    pub home_team: String,
    #[serde(rename = "Away_team", default)]
    pub away_team: String,
    #[serde(rename = "GameId", default)]
    pub game_id: String,
    #[serde(rename = "HomeTeamScore", default)]
    pub home_team_score: u32,
    #[serde(rename = "VisitorTeamScore", default)]
    pub visitor_team_score: u32,
    #[serde(rename = "GameDate", default)]
    pub game_date: String,
    #[serde(rename = "HomeTeamId")]
    pub home_team_id: u64,
    #[serde(rename = "AwayTeamId")]
    pub away_team_id: u64,
    #[serde(rename = "Home_team_abbr", default)]
    pub home_team_abbr: String,
    #[serde(rename = "Away_team_abbr", default)]
    pub away_team_abbr: String,
    #[serde(rename = "L2M_Comments")]
    pub l2m_comments: serde_json::Value,
    #[serde(rename = "GameDateOut", default)]
    pub game_date_out: String,
}

impl std::default::Default for LastTwoMinutesGame {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LastTwoMinutesStat {
    #[serde(default)]
    pub stats_name: String,
    #[serde(default)]
    pub home: u32,
    #[serde(default)]
    pub away: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LastTwoMinutesEntry {
    #[serde(rename = "PeriodName", default)]
    pub period_name: String,
    #[serde(rename = "PCTime", default)]
    pub pc_time: String,
    #[serde(rename = "ImposibleIndicator")]
    pub imposible_indicator: u32,
    #[serde(rename = "Comment", default)]
    pub comment: Option<String>,
    #[serde(rename = "CallRatingName", default)]
    pub call_rating_name: String,
    #[serde(rename = "CallType", default)]
    pub call_type: String,
    #[serde(rename = "CP")]
    pub cp: Option<String>,
    #[serde(rename = "DP")]
    pub dp: Option<String>,
    #[serde(rename = "Difficulty", default)]
    pub difficulty: String,
    #[serde(rename = "VideolLink", default)]
    pub video_link: String,
    #[serde(rename = "Qualifier", default)]
    pub qualifier: serde_json::Value,
    #[serde(rename = "posID", default)]
    pub pos_id: u32,
    #[serde(rename = "posStart", default)]
    pub pos_start: String,
    #[serde(rename = "posEnd", default)]
    pub pos_end: String,
    #[serde(rename = "posTeamId", default)]
    pub pos_team_id: u32,
    #[serde(rename = "teamIdInFavor")]
    pub team_id_in_favor: serde_json::Value,
    #[serde(rename = "errorInFavor", default)]
    pub error_in_favor: String,
    #[serde(rename = "imgChart")]
    pub img_chart: f32,
}

pub async fn get_schedule(season_year: u32) -> Result<DataNbaNetResponse<Game>, String> {
    get_data_nba_net_response::<Game>(&format!("{season_year}/schedule.json")).await
}

pub async fn get_teams(season_year: u32) -> Result<DataNbaNetResponse<Team>, String> {
    get_data_nba_net_response::<Team>(&format!("{season_year}/teams.json")).await
}

pub async fn get_data_nba_net_response<T: Default + DeserializeOwned>(
    endpoint: &str,
) -> Result<DataNbaNetResponse<T>, String> {
    let url = format!("http://data.nba.net/prod/v2/{endpoint}");
    get_json_data(&url).await
}

pub async fn get_json_data<T: Default + DeserializeOwned>(url: &str) -> Result<T, String> {
    let response = reqwest::get(url).await.unwrap();
    let status = response.status();
    let body_str = response.text().await.unwrap();
    if !status.is_success() {
        return Err(format!(
            "{}\nError status: {} {}",
            body_str,
            status.as_u16(),
            status.as_str()
        ));
    }
    let body: T = serde_json::from_str(&body_str)
        .map_err(|e| {
            panic!("{body_str}\n{e}");
        })
        .unwrap();
    Ok(body)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataNbaNetResponse<T: Default> {
    pub _internal: serde_json::Value,
    pub league: DataNbaLeagues<T>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataNbaLeagues<T> {
    #[serde(default)]
    pub standard: Vec<T>,
    #[serde(default)]
    pub africa: Vec<T>,
    #[serde(default)]
    pub sacramento: Vec<T>,
    #[serde(default)]
    pub vegas: Vec<T>,
    #[serde(default)]
    pub utah: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_id: String,
    pub season_stage_id: u32,
    pub game_url_code: String,
    pub status_num: u32,
    pub extended_status_num: u32,
    #[serde(rename = "isStartTimeTBD")]
    pub is_start_time_tbd: bool,
    #[serde(rename = "startTimeUTC", with = "time::serde::iso8601")]
    pub start_time_utc: time::OffsetDateTime,
    pub start_date_eastern: String,
    pub is_neutral_venue: bool,
    pub start_time_eastern: String,
    pub is_buzzer_beater: bool,
    pub period: GamePeriod,
    #[serde(default)]
    pub nugget: Option<GameNugget>,
    pub h_team: GameTeam,
    pub v_team: GameTeam,
    pub watch: GameWatchDetails,
}

impl std::default::Default for Game {
    fn default() -> Self {
        todo!();
        // Self {

        // }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePeriod {
    pub current: u8,
    #[serde(rename = "type")]
    pub kind: u32,
    pub max_regular: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameNugget {
    pub text: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameTeam {
    pub team_id: String,
    pub score: String,
    pub win: String,
    pub loss: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameWatchDetails {
    pub broadcast: GameWatchBroadcast,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameWatchBroadcast {
    pub video: GameWatchVideo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameWatchVideo {
    pub regional_blackout_codes: String,
    pub is_league_pass: bool,
    pub is_national_blackout: bool,
    #[serde(rename = "isTNTOT")]
    pub is_tnt_ot: bool,
    pub can_purchase: bool,
    #[serde(rename = "isVR")]
    pub is_vr: bool,
    #[serde(rename = "isNextVR")]
    pub is_next_vr: bool,
    #[serde(rename = "isNBAOnTNTVR")]
    pub is_nba_on_tnt_vr: bool,
    pub is_magic_leap: bool,
    pub is_oculus_venues: bool,
    pub national: NationalWatch,
    pub canadian: Vec<serde_json::Value>,
    #[serde(rename = "spanish_national")]
    pub spanish_national: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NationalWatch {
    pub broadcasters: Vec<Broadcaster>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Broadcaster {
    pub short_name: String,
    pub long_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[serde(rename = "isNBAFranchise", default)]
    pub is_nba_franchise: bool,
    pub is_all_star: bool,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub alt_city_name: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub tricode: String,
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub url_name: String,
    #[serde(default)]
    pub team_short_name: String,
    #[serde(default)]
    pub conf_name: String,
    #[serde(default)]
    pub div_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
}
