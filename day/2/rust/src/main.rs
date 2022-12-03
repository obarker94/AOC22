use std::collections::HashMap;

#[derive(Eq, PartialEq)]
struct Outcomes {
    X: &'static str,
    Z: &'static str,
    value: i32,
}

#[derive(Eq, PartialEq)]
struct GameMatrix {
    A: Outcomes,
    B: Outcomes,
    C: Outcomes,
}

fn return_strategy_move(opponent_move: &str, my_move: &str) -> i32 {
    let game_choice_matrix = GameMatrix {
        A: Outcomes {
            X: "C",
            Z: "B",
            value: 1,
        },
        B: Outcomes {
            X: "A",
            Z: "C",
            value: 2,
        },
        C: Outcomes {
            X: "B",
            Z: "A",
            value: 3,
        },
    };

    let mut game_choice_hashmap = HashMap::new();
    game_choice_hashmap.insert("A", game_choice_matrix.A);
    game_choice_hashmap.insert("B", game_choice_matrix.B);
    game_choice_hashmap.insert("C", game_choice_matrix.C);

    if my_move == "Y" {
        game_choice_hashmap[opponent_move].value + 3
    } else if my_move == "X" {
        game_choice_hashmap[game_choice_hashmap[opponent_move].X].value
    } else if my_move == "Z" {
        game_choice_hashmap[game_choice_hashmap[opponent_move].Z].value + 6
    } else {
        0
    }
}

fn main() {
    let data = std::fs::read_to_string("data.txt").expect("Error reading file");
    let parsed_lines = data.lines().collect::<Vec<&str>>();

    let parsed_rounds = parsed_lines
        .iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let processed_rounds = parsed_rounds
        .iter()
        .map(|x| {
            let player1 = x[0];
            let player2 = x[1];
            let round_result = return_strategy_move(player1, player2);
            round_result
        })
        .collect::<Vec<i32>>();

    let total_score = processed_rounds.iter().sum::<i32>();

    println!("Test : {:?}", total_score);
}
