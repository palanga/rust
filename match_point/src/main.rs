#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::{env, io};
use std::fmt::{Display, Formatter};

fn main() {
    let player_names = get_player_names_from_command_line_args();

    if player_names.is_empty() { panic!("tenés que pasar los nombres de las personas separados por espacios") }

    let mut game = InMemoryGame::of(player_names);

    loop {
        println!("###########################################################");
        println!("la partida va así:");
        println!("{}", game);
        println!("le toca a {}", game.current_player());
        println!("cuántos puntos hizo en este turno?");

        let mut points = String::new();

        io::stdin()
            .read_line(&mut points)
            .expect("no se pudo leer");

        let points: i32 = match points.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!();
                println!("escribí un numero!");
                println!();
                continue;
            }
        };

        game.add_points_current_player(points);
        println!();
    }
}

fn get_player_names_from_command_line_args() -> Vec<String> {
    let mut args = env::args();

    args.next(); // discard program name

    args.collect()
}

trait Game {
    fn current_player(&self) -> String;
    fn add_points_current_player(&mut self, points: i32);
}

struct InMemoryGame {
    players: HashMap<String, RefCell<Player>>,
    players_ring: Vec<String>,
    turns_played: i32,
}

impl Display for InMemoryGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let players_display: Vec<&RefCell<Player>> =
            self.players
                .iter()
                .map(|item| item.1)
                .collect();

        players_display.iter().for_each(|player| write!(f, "{}\n", player.borrow()).unwrap());


        Ok(())

    }
}

impl InMemoryGame {
    pub fn of(player_names: Vec<String>) -> InMemoryGame {
        let players =
            player_names
                .iter()
                .map(|name| (name.clone(), Player::of(name)))
                .collect();

        InMemoryGame {
            players,
            players_ring: player_names,
            turns_played: 0,
        }
    }
}

impl Game for InMemoryGame {
    fn current_player(&self) -> String {
        let index = self.turns_played as usize % self.players_ring.len();
        self.players_ring[index].clone()
    }

    fn add_points_current_player(&mut self, points: i32) {
        let player_name = self.current_player();
        let mut player = self.players[&player_name].borrow_mut();
        player.points += points;
        self.turns_played += 1;
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    points: i32,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.points)
    }
}

impl Player {
    pub fn of(name: &String) -> RefCell<Player> {
        RefCell::new(Player {
            name: name.clone(),
            points: 0,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn a_game_with_two_players_starts_with_zero_points_each() {
        let nube = String::from("nube");
        let cuba = String::from("cuba");
        let game = InMemoryGame::of(vec![nube.clone(), cuba.clone()]);
        assert_eq!(game.players[&nube].borrow().points, 0);
        assert_eq!(game.players[&cuba].borrow().points, 0);
    }

    #[test]
    fn points_can_be_added_to_a_player() {
        let nube = String::from("nube");
        let mut game = InMemoryGame::of(vec![nube.clone()]);
        game.add_points_current_player(17);
        assert_eq!(game.players[&nube].borrow().points, 17);
    }

    #[test]
    fn first_current_player_of_the_game() {
        let nube = String::from("nube");
        let cuba = String::from("cuba");
        let game = InMemoryGame::of(vec![nube.clone(), cuba.clone()]);
        assert_eq!(game.current_player(), nube);
    }

    #[test]
    fn when_points_are_added_to_the_current_player_then_it_is_next_player_turn() {
        let nube = String::from("nube");
        let cuba = String::from("cuba");
        let mut game = InMemoryGame::of(vec![nube.clone(), cuba.clone()]);
        assert_eq!(game.current_player(), nube);
        game.add_points_current_player(17);
        assert_eq!(game.current_player(), cuba);
    }

    #[test]
    fn points_can_be_added_to_multiple_players() {
        let nube = String::from("nube");
        let cuba = String::from("cuba");
        let mut game = InMemoryGame::of(vec![nube.clone(), cuba.clone()]);
        game.add_points_current_player(17);
        game.add_points_current_player(21);
        assert_eq!(game.players[&nube].borrow().points, 17);
        assert_eq!(game.players[&cuba].borrow().points, 21);
    }

    #[test]
    fn when_all_players_have_played_then_the_round_restarts() {
        let nube = String::from("nube");
        let cuba = String::from("cuba");
        let mut game = InMemoryGame::of(vec![nube.clone(), cuba.clone()]);
        game.add_points_current_player(17);
        game.add_points_current_player(21);
        assert_eq!(game.current_player(), nube);
    }
}
