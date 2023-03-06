use std::{collections::{HashMap}, fs::File, io::Write};

use nba_utils::{Team};
use progress_string::{Bar, BarBuilder};

struct GameIterProgress {
    message: String,
    bar: Bar,
}

impl GameIterProgress {
    pub fn new(total: usize) -> Self {
        let bar = BarBuilder::new()
            .total(total)
            .include_percent()
            .include_numbers()
            .empty_char('🮖')
            .full_char('🮗')
            .leading_char('>')
            .build();
        let (terminal_width, _terminal_height) = termion::terminal_size().unwrap();
        let message = " ".repeat((terminal_width - 1) as usize);
        
        println!(
            "{}",
            termion::cursor::Save,
        );
        Self {
            message,
            bar,
        }
    }
    pub fn start(&mut self, name: &str) {
        self.bar.update(1);
        self.update_message(format!("downloading {}", name));
    }
    pub fn complete(&mut self, name: &str) {
        self.update_message(format!("completed {}",  name));
    }
    fn update_message(&mut self, msg: String) {
        let last_length = self.message.len();
        self.message = msg;
        while self.message.len() < last_length {
            self.message.push(' ');
        }
    }
    fn write(&self) {
        print!(
            "{}{}{}\n{}",
            termion::cursor::Up(1),
            termion::cursor::Left(self.bar.get_last_width() as u16),
            self.message,
            self.bar.to_string(),
        );
    }
}

impl Drop for GameIterProgress {
    fn drop(&mut self) {
        println!(
            "\n{}",
            termion::cursor::Restore,
        )
    }
}

#[derive(Default, Clone)]
struct L2MDetails {
    pub team: Team,
    pub in_favor: u32,
    pub against: u32,
    pub missed_games: u32,
}

impl std::fmt::Debug for L2MDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("L2MDetails")
            .field("team", &self.team.nickname)
            .field("in_favor", &self.in_favor)
            .field("against", &self.against)
            .field("missed_games", &self.missed_games)
            .finish()
    }
}

#[tokio::main]
async fn main() {
    let teams = nba_utils::get_teams(2022).await.unwrap();
    let mut teams: HashMap<String, L2MDetails> = teams
    .league
        .standard
        .into_iter()
        .map(|t| (t.team_id.clone(), L2MDetails {
            team: t,
            ..Default::default()
        }))
        .collect();
    let now = time::OffsetDateTime::now_utc();
    let sched = nba_utils::get_schedule(2022).await.unwrap();
    let two_minutes_games: Vec<_> = sched
        .league
        .standard
        .into_iter()
        .filter(|g| g.season_stage_id == 2 && g.start_time_utc < now)
        .collect();
    let mut bar = GameIterProgress::new(two_minutes_games.len());
    for game in two_minutes_games {
        let game_name = {
            let home = teams.get(&game.h_team.team_id).unwrap();
            let away = teams.get(&game.v_team.team_id).unwrap();
            format!("{} v {}", home.team.tricode, away.team.tricode)
        };
        bar.start(&game_name);
        bar.write();
        
        let last_two = match nba_utils::get_last_two_minutes(&game.game_id).await {
            Ok(last_two) => last_two,
            Err(_e) => {
                teams.entry(game.h_team.team_id.clone()).and_modify(|ent| {
                    ent.missed_games += 1;
                });
                teams.entry(game.v_team.team_id.clone()).and_modify(|ent| {
                    ent.missed_games += 1;
                });
                continue;
            }
        };
        let mut home_help = 0;
        let mut home_hurt = 0;
        let mut away_help = 0;
        let mut away_hurt = 0;
        for stats in &last_two.stats {
            if stats.stats_name == "Errors in Favor" {
                home_help += stats.home;
                away_hurt += stats.home;
                home_hurt += stats.away;
                away_help += stats.away;
            }
        }
        teams.entry(game.h_team.team_id.clone()).and_modify(|ent| {
            ent.against += home_hurt;
            ent.in_favor += home_help;
        });
        teams.entry(game.v_team.team_id.clone()).and_modify(|ent| {
            ent.against += away_hurt;
            ent.in_favor += away_help;
        });
        bar.complete(&game_name);
        bar.write();
    }
    
    let mut file = File::create("./out.csv").unwrap();
    file.write_all(b"team, in_favor, against, missed games\n").unwrap();
    for (_id, dets) in teams {
        if !dets.team.is_nba_franchise {
            continue;
        }
        file.write_all(
            format!("{},{},{},{}\n", dets.team.tricode, dets.in_favor, dets.against, dets.missed_games).as_bytes()
        ).unwrap();
    }
}
