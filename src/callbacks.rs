//use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Entity, World};

pub type HtmlCallback = dyn Fn(&mut World, Entity) + Send + Sync + 'static;
