use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Args {
    /// List of players names
    #[arg(short, long, value_delimiter = ',')]
    pub players: Option<Vec<String>>,

    /// Location of the players folder
    #[arg(
        short,
        long,
        default_value = "/players"
    )]
    pub folder: PathBuf,

    #[arg(short, long, value_delimiter = ',')]
    pub create_player: Option<Vec<String>>,
}
