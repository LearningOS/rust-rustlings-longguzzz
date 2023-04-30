// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.


// use std::borrow::Cow;
// use std::collections::HashMap;

// // A structure to store team name and its goal details.
// struct Team {
//     goals_scored: u8,
//     goals_conceded: u8,
// }

// // fn build_scores_table(results: String) -> HashMap<String, Team> {
// fn build_scores_table(results: String) -> HashMap<Cow<'static, str>, Team> {
//     // The name of the team is the key and its associated struct is the value.
//     // let mut scores: HashMap<String, Team> = HashMap::new();
//     let mut scores: HashMap<Cow<str>, Team> = HashMap::new();

//     for r in results.lines() {
//         let v: Vec<&str> = r.split(',').collect();
//         let team_1_name = v[0].to_string();
//         let team_1_score: u8 = v[2].parse().unwrap();
//         let team_2_name = v[1].to_string();
//         let team_2_score: u8 = v[3].parse().unwrap();
//         // TODO: Populate the scores table with details extracted from the
//         // current line. Keep in mind that goals scored by team_1
//         // will be number of goals conceded from team_2, and similarly
//         // goals scored by team_2 will be the number of goals conceded by
//         // team_1.
//         // let team_1 = scores.entry(team_1_name).or_insert(Team {

//         // 分析1：问题在于team_1_name的信息其实重复记录了
//         // 分析2：想要减少一次String的内存分配。借用name，查询，没有对应的值则移动进hashmap
//             // 这种做法的问题会导致借用到一半的时候被move，如果硬要做，可能得unsafe
//         let team_1_name_ref: &str = &team_1_name;
//         match scores.get_mut(team_1_name_ref) {
//             Some(existing_team) => {
//                 existing_team.goals_scored += team_1_score;
//                 existing_team.goals_conceded += team_1_score;
//             },
//             None => {
//                 let new_team = Team {
//                     name: team_1_name,
//                     goals_scored: team_1_score,
//                     goals_conceded: team_1_score,
//                 };
//                 let name_ref = &new_team.name;
//                 scores.insert(Cow::Borrowed(name_ref), new_team);
//             }
//         };
//         // let team_1 = scores.entry(Cow::Borrowed(team_1_name_ref)).or_insert(Team {
//         //     name: team_1_name,
//         //     goals_scored: 0,
//         //     goals_conceded: 0,
//         // });
//         // team_1.goals_scored += team_1_score;
//         // team_1.goals_conceded += team_1_score;

        
//         // let team_2 = scores.entry(team_2_name).or_insert(Team {
//         // let team_2 = scores.entry(Cow::Borrowed(&*<String as Into<&str>>::into(team_2_name))).or_insert(Team {
//         let team_2_name_ref: &str = &team_2_name;
//         let team_2 = scores.entry(Cow::Borrowed(team_2_name_ref)).or_insert(Team {
//             name: team_2_name,
//             goals_scored: 0,
//             goals_conceded: 0,
//         });
//         team_2.goals_scored += team_2_score;
        
//     }
//     scores
// }



use std::borrow::Cow;
use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        let team_1 = scores.entry(team_1_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;
        
        let team_2 = scores.entry(team_2_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
        
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        // let mut keys: Vec<&String> = scores.keys().map(|cow| {
        //     match cow {
        //         Cow::Borrowed(_) => panic!("Unexpected Cow::Borrowed variant"),
        //         Cow::Owned(s) => s,
        //     }
        // }).collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
