use bevy::prelude::{Entity, World};

use crate::error::HtmlUiError;

pub type HtmlCallback =
	dyn Fn(&mut World, Entity) -> Result<(), HtmlUiError> + Send + Sync + 'static;
