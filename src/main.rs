mod args;
pub mod champ_pool;
mod file_manipulation;
mod player;
pub mod position;

use clap::Parser;
use file_manipulation::create_player;
use position::Position::*;
use rand::prelude::*;
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::PathBuf;

use self::args::Args;
use self::champ_pool::ChampPool;
use self::player::Player;

fn main() -> Result<(), String> {
    let args = Args::parse();

    if let Some(pl) = args.create_player {
        for p in pl {
            create_player(&args.folder, &p)?
        }
    }

    if let Some(pl) = args.players {
        ultimate_cowardice(pl, args.folder)?
    }

    Ok(())
}

fn choose_random<T>(vec: &mut Vec<T>) -> Option<T> {
    let i = (0..vec.len()).choose(&mut thread_rng())?;
    Some(vec.swap_remove(i))
}

fn ultimate_cowardice(players: Vec<String>, folder: PathBuf) -> Result<(), String> {
    let mut available_roles = vec![Top, Jungle, Mid, Bottom, Support];

    if players.len() > 5 {
        println!("League of legends cannot be played with more than 5 player a team");
        return Ok(());
    }

    let mut pl: Vec<Player> = Vec::new();

    for player_name in players {
        let mut t = folder.clone();
        t.push(format!("{}.json", &player_name.trim().to_lowercase()));
        let champ_pool: ChampPool = from_str(
            &read_to_string(&t).map_err(|_| format!("File {} not found", t.to_string_lossy()))?,
        )
        .map_err(|_| format!("Failed to parse the file {}", t.to_string_lossy()))?;
        let role =
            choose_random(&mut available_roles).expect("Seems like my bounds check failed :   ^)");
        let mut champion_list = match role {
            Top => champ_pool.top,
            Jungle => champ_pool.jng,
            Mid => champ_pool.mid,
            Bottom => champ_pool.bot,
            Support => champ_pool.sup,
        };

        match choose_random(&mut champion_list) {
            Some(champion) => pl.push(Player {
                name: player_name.to_owned(),
                position: role,
                champion,
            }),
            None => {
                println!(
                    "The player {} is a Wukong irl, and cannot play a champion from their role",
                    player_name
                );
                return Ok(());
            }
        }
    }
    pl.iter()
        .for_each(|p| println!("{} IN {:?} WITH {}", p.name, p.position, p.champion));
    Ok(())
}
