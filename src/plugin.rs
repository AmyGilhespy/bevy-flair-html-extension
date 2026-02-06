use bevy::{
	asset::{LoadState, RecursiveDependencyLoadState},
	ecs::message::{MessageCursor, Messages},
	prelude::*,
};
use bevy_flair::style::StyleSheet;

use crate::{
	asset::HtmlUiAsset,
	build::{HtmlUiRoot, spawn_html_ui},
	loader::HtmlUiLoader,
	resources::HtmlCssUiResource,
};

pub struct HtmlUiPlugin;

impl Plugin for HtmlUiPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset::<HtmlUiAsset>()
			.register_asset_loader(HtmlUiLoader)
			.add_systems(Update, html_ui_hot_reload)
			.add_systems(Update, html_ui_watch_load);
	}
}

fn html_ui_hot_reload(
	world: &mut World,
	mut cursor: Local<MessageCursor<AssetEvent<HtmlUiAsset>>>,
) {
	let mut to_rebuild = Vec::new();

	{
		let mut q_roots = world.query::<(Entity, &HtmlUiRoot)>();

		let messages = world
			.get_resource::<Messages<AssetEvent<HtmlUiAsset>>>()
			.expect("AssetEvent messages missing");

		for event in cursor.read(messages) {
			let AssetEvent::Modified { id } = event else {
				continue;
			};

			for (entity, root) in q_roots.iter(world) {
				if root.id == *id {
					to_rebuild.push((entity, *id));
				}
			}
		}
	}

	for (entity, id) in to_rebuild {
		// Not sure if these commented-out checks were needed.  Needs testing.
		//if world.resource::<Assets<HtmlUiAsset>>().get(id).is_some() && world.get_resource::<HtmlCssUiResource>().is_some() {
		world.despawn(entity);
		let _ = spawn_html_ui(world, id);
		//}
	}
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_lines)]
fn html_ui_watch_load(
	world: &mut World,
	mut loaded_handles_ids: Local<Option<(AssetId<HtmlUiAsset>, Option<AssetId<StyleSheet>>)>>,
	mut root_entity: Local<Option<Entity>>,
) {
	let id_html: AssetId<HtmlUiAsset>;
	{
		// ---- Phase 1: Check if we should skip or despawn old UI ----
		let Some(res) = world.get_resource::<HtmlCssUiResource>() else {
			// Resource doesn't exist, despawn any existing UI
			if let Some(re) = *root_entity {
				let mut q_roots = world.query::<(Entity, &HtmlUiRoot)>();
				let mut entities_to_despawn = Vec::new();
				for (entity, _) in q_roots.iter(world) {
					if entity == re {
						entities_to_despawn.push(entity);
					}
				}
				for entity in entities_to_despawn {
					world.despawn(entity);
				}
				*root_entity = None;
			}
			*loaded_handles_ids = None;
			return;
		};

		// Check if assets have changed
		id_html = res.html.id();
		let mut needs_reload = true;

		if let Some(loaded_ids) = *loaded_handles_ids {
			let same_html = id_html == loaded_ids.0;
			let same_css = match &res.css {
				Some(css) => Some(css.id()) == loaded_ids.1,
				None => loaded_ids.1.is_none(),
			};

			if same_html && same_css {
				needs_reload = false;
			}
		}

		if !needs_reload {
			return;
		}

		// ---- Phase 2: Check load states ----
		let asset_server = world.resource::<AssetServer>();

		// Check HTML load state
		let Some(load_state_html) = asset_server.get_load_state(id_html) else {
			return;
		};
		match load_state_html {
			LoadState::Loaded => {}
			LoadState::NotLoaded | LoadState::Loading | LoadState::Failed(_) => return,
		}

		// Check HTML recursive dependencies
		let Some(recursive_load_state_html) =
			asset_server.get_recursive_dependency_load_state(id_html)
		else {
			return;
		};
		match recursive_load_state_html {
			RecursiveDependencyLoadState::Loaded => {}
			_ => return,
		}

		// Check HTML asset exists
		if world
			.resource::<Assets<HtmlUiAsset>>()
			.get(id_html)
			.is_none()
		{
			return;
		}

		// Check CSS if present
		if let Some(res_css) = &res.css {
			let id_css = res_css.id();

			// Check CSS load state
			let Some(load_state_css) = asset_server.get_load_state(id_css) else {
				return;
			};
			match load_state_css {
				LoadState::Loaded => {}
				_ => return,
			}

			// Check CSS recursive dependencies
			let Some(recursive_load_state_css) =
				asset_server.get_recursive_dependency_load_state(id_css)
			else {
				return;
			};
			match recursive_load_state_css {
				RecursiveDependencyLoadState::Loaded => {}
				_ => return,
			}

			// Check CSS asset exists
			let assets_css = world.resource::<Assets<StyleSheet>>();
			if assets_css.get(id_css).is_none() {
				return;
			}
		}

		// ---- Phase 3: Despawn old UI ----
		if let Some(re) = *root_entity {
			let mut q_roots = world.query::<(Entity, &HtmlUiRoot)>();
			let mut entities_to_despawn = Vec::new();
			for (entity, _) in q_roots.iter(world) {
				if entity == re {
					entities_to_despawn.push(entity);
				}
			}
			for entity in entities_to_despawn {
				world.despawn(entity);
			}
			*root_entity = None;
		}
	}

	// ---- Phase 4: Spawn new UI ----
	match spawn_html_ui(world, id_html) {
		Ok(new_root) => {
			*root_entity = Some(new_root);
			if let Some(res) = world.get_resource::<HtmlCssUiResource>() {
				*loaded_handles_ids =
					Some((id_html, res.css.as_ref().map(bevy::prelude::Handle::id)));
			} else {
				*loaded_handles_ids = Some((id_html, None));
			}
		}
		Err(_) => {
			*loaded_handles_ids = None;
		}
	}
}
