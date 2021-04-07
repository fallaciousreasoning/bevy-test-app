use bevy::prelude::Query;
use super::components::Position;

pub fn mover(query: Query<&Position>) {
    for pos in query.iter() {
        println!("{:?}", pos);
    }
}