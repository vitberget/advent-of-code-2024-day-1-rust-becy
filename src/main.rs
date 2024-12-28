use std::fs::read_to_string;

use bevy::prelude::*;
use render::objects::add_objects;
use render::player::add_player;
use render::puzzle::{escape_the_matrix, setup_puzzle_ticker, smooth_object, smooth_player, step_trigger};
use render::score::{score_trigger, setup_score};
use render::setup_things;
use render::walls::add_walls;
use render::floor::add_floor;
use warehouse::structs::Warehouse;

pub mod warehouse;
pub mod render;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PuzzleState {
    #[default]
    Solving,
    Scoring,
    Completed
}

fn main() -> anyhow::Result<()> {
   let text = read_to_string("puzzle.txt")?;
   let warehouse = Warehouse::parse(&text)?;

   App::new()
       .add_plugins(DefaultPlugins)
       .init_state::<PuzzleState>()
       .insert_resource(warehouse)
       .add_systems(Startup, (setup_things, add_floor, add_walls, add_objects, add_player, setup_puzzle_ticker))
       .add_systems(Update, (step_trigger, smooth_object, smooth_player, escape_the_matrix).run_if(in_state(PuzzleState::Solving)))
       .add_systems(Update, (score_trigger).run_if(in_state(PuzzleState::Scoring)))
       .add_systems(OnEnter(PuzzleState::Scoring), setup_score)
       .run();

    Ok(())
}
