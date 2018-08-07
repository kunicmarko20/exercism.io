use std::collections::HashMap;

pub struct Team {
    name: String,
    matches_played: u32,
    won: u32,
    draw: u32,
    lost: u32,
    points: u32,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            name,
            matches_played: 0,
            won: 0,
            draw: 0,
            lost: 0,
            points: 0,
        }
    }

    pub fn won(&mut self) {
        self.won += 1;
        self.points += 3;
        self.matches_played += 1;
    }

    pub fn lost(&mut self) {
        self.lost += 1;
        self.matches_played += 1;
    }

    pub fn draw(&mut self) {
        self.draw += 1;
        self.points +=1;
        self.matches_played += 1;
    }

    pub fn to_string(&self) -> String {
        format!(
            "\n{} |  {} | {} | {} | {} | {}",
            self.name,
            self.matches_played,
            self.won,
            self.draw,
            self.lost,
            self.points
        )
    }
}

pub fn tally(matches_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();

    for result in matches_results.split("\n") {
        let data = result.split(";").collect::<Vec<&str>>();
        let mut team1 = teams.entry(data[0].to_string()).or_insert(Team::new(data[0].to_string()));
        let mut team2 = teams.entry(data[1].to_string()).or_insert(Team::new(data[1].to_string()));

        match data[2] {
            "win" => { team1.won(); team2.lost() },
            "lost" => { team1.lost(); team2.won() },
            "draw" => { team1.draw(); team2.draw(); },
            _ => {}
        }
    }

    let mut output = "Team                           | MP |  W |  D |  L |  P\n".to_string();

    for (_, team) in teams {
        output += &team.to_string();
    }

    output
}
