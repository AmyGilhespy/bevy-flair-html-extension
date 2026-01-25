use bevy::prelude::{Entity, World};

pub type HtmlCallback = dyn Fn(&mut World, Entity) -> Option<()> + Send + Sync + 'static;
