use bevy::{
	asset::{LoadState, RecursiveDependencyLoadState},
	prelude::*,
};
use bevy_flair::style::{StyleSheet, components::NodeStyleSheet};

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

#[allow(clippy::needless_pass_by_value)]
fn html_ui_hot_reload(
	mut messages: MessageReader<AssetEvent<HtmlUiAsset>>,
	assets_html: Res<Assets<HtmlUiAsset>>,
	mut commands: Commands,
	q_roots: Query<(Entity, &HtmlUiRoot)>,
	handles: Option<Res<HtmlCssUiResource>>,
) {
	for message in messages.read() {
		if let AssetEvent::Modified { id } = message
			&& let Some(asset_html_actual) = assets_html.get(*id)
		{
			for (entity, root) in q_roots.iter() {
				if root.id == *id {
					let mut style_sheet = NodeStyleSheet::Inherited;
					if let Some(res) = &handles
						&& let Some(res_css) = &res.css
					{
						style_sheet = NodeStyleSheet::new(res_css.clone());
					}
					commands.entity(entity).despawn();
					let _ = spawn_html_ui(&mut commands, asset_html_actual, *id, style_sheet);
				}
			}
		}
	}
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn html_ui_watch_load(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	assets_html: Res<Assets<HtmlUiAsset>>,
	assets_css: Res<Assets<StyleSheet>>,
	handles: Option<Res<HtmlCssUiResource>>,
	mut loaded_handles_ids: Local<Option<(AssetId<HtmlUiAsset>, Option<AssetId<StyleSheet>>)>>,
	mut root_entity: Local<Option<Entity>>,
	q_roots: Query<(Entity, &HtmlUiRoot)>,
) {
	let Some(res) = handles else {
		if loaded_handles_ids.is_some()
			&& let Some(re) = *root_entity
		{
			for (e, _) in q_roots.iter() {
				if e == re {
					commands.entity(e).despawn();
				}
			}
		}
		return;
	};

	let id_html = res.html.id();
	if let Some(loaded_ids) = *loaded_handles_ids {
		if let Some(res_css) = &res.css {
			let id_css = res_css.id();
			if id_html == loaded_ids.0 && Some(id_css) == loaded_ids.1 {
				return;
			}
		} else if id_html == loaded_ids.0 && loaded_ids.1.is_none() {
			return;
		}
	}

	let Some(load_state_html) = asset_server.get_load_state(id_html) else {
		return;
	};
	match load_state_html {
		LoadState::NotLoaded | LoadState::Loading | LoadState::Failed(_) => return,
		LoadState::Loaded => {}
	}
	let Some(load_state_recursive_html) = asset_server.get_recursive_dependency_load_state(id_html)
	else {
		return;
	};
	match load_state_recursive_html {
		RecursiveDependencyLoadState::NotLoaded
		| RecursiveDependencyLoadState::Loading
		| RecursiveDependencyLoadState::Failed(_) => return,
		RecursiveDependencyLoadState::Loaded => {}
	}

	let opt_html = assets_html.get(id_html);
	let Some(asset_html_actual) = opt_html else {
		return; // Not loaded yet.
	};

	if let Some(res_css) = &res.css {
		let id_css = res_css.id();

		let Some(load_state_css) = asset_server.get_load_state(id_css) else {
			return;
		};
		match load_state_css {
			LoadState::NotLoaded | LoadState::Loading | LoadState::Failed(_) => return,
			LoadState::Loaded => {}
		}
		let Some(load_state_recursive_css) =
			asset_server.get_recursive_dependency_load_state(id_css)
		else {
			return;
		};
		match load_state_recursive_css {
			RecursiveDependencyLoadState::NotLoaded
			| RecursiveDependencyLoadState::Loading
			| RecursiveDependencyLoadState::Failed(_) => return,
			RecursiveDependencyLoadState::Loaded => {}
		}

		let opt_css = assets_css.get(id_css);
		if opt_css.is_none() {
			return; // Not loaded yet.
		}
	}

	if let Some(re) = *root_entity {
		for (e, _) in q_roots.iter() {
			if e == re {
				commands.entity(e).despawn();
			}
		}
	}

	let mut style_sheet = NodeStyleSheet::Inherited;
	let mut id_css_opt = None;
	if let Some(res_css) = &res.css {
		id_css_opt = Some(res_css.id());
		style_sheet = NodeStyleSheet::new(res_css.clone());
	}

	*root_entity = Some(spawn_html_ui(
		&mut commands,
		asset_html_actual,
		id_html,
		style_sheet,
	));

	*loaded_handles_ids = Some((id_html, id_css_opt));
}
