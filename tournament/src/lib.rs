use std::{collections::HashMap, ops::AddAssign};
pub fn tally(match_results: &str) -> String {
    let mut wons = HashMap::new();
    let mut draws = HashMap::new();
    let mut losses = HashMap::new();
    let mut points = HashMap::new();
    for line in match_results.lines() {
        let chunks: Vec<_> = line.split(';').collect();
        let a = chunks[0];
        let b = chunks[1];
        let result = chunks[2];
        match result {
            "win" => {
                wons.entry(a).or_insert(0).add_assign(1);
                losses.entry(b).or_insert(0).add_assign(1);
                points.entry(a).or_insert(0).add_assign(3);
                points.entry(b).or_insert(0);
            }
            "loss" => {
                losses.entry(a).or_insert(0).add_assign(1);
                wons.entry(b).or_insert(0).add_assign(1);
                points.entry(a).or_insert(0);
                points.entry(b).or_insert(0).add_assign(3);
            }
            "draw" => {
                draws.entry(a).or_insert(0).add_assign(1);
                draws.entry(b).or_insert(0).add_assign(1);
                points.entry(a).or_insert(0).add_assign(1);
                points.entry(b).or_insert(0).add_assign(1);
            }
            _ => panic!("Unexpected result"),
        }
    }

    let mut sorted_teams: Vec<_> = points.keys().cloned().collect();
    sorted_teams.sort_by(
        |a, b| match points.get(*a).unwrap().cmp(points.get(*b).unwrap()) {
            std::cmp::Ordering::Equal => a.cmp(b),
            x => x.reverse(),
        },
    );

    let mut s = String::new();
    s += &format!(
        "{:<31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
        "Team", "MP", "W", "D", "L", "P",
    );
    for team in sorted_teams.into_iter() {
        let won = wons.get(team).unwrap_or(&0);
        let draw = draws.get(team).unwrap_or(&0);
        let loss = losses.get(team).unwrap_or(&0);
        let match_played = won + draw + loss;
        let point = points.get(team).unwrap_or(&0);
        s += &format!(
            "\n{:<31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
            team, match_played, won, draw, loss, point
        );
    }
    s
}
