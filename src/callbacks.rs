use anyhow::Result;
use bevy::prelude::{Entity, World};

pub type HtmlCallback = dyn Fn(&mut World, Entity) -> Result<()> + Send + Sync + 'static;
