use bevy::prelude::*;
use bevy_flair::style::components::NodeStyleSheet;

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
	assets: Res<Assets<HtmlUiAsset>>,
	mut commands: Commands,
	q_roots: Query<(Entity, &HtmlUiRoot)>,
	handles: Option<Res<HtmlCssUiResource>>,
) {
	for message in messages.read() {
		if let AssetEvent::Modified { id } = message
			&& let Some(asset) = assets.get(*id)
		{
			for (entity, root) in q_roots.iter() {
				if root.id == *id {
					commands.entity(entity).despawn();
					let entity = spawn_html_ui(&mut commands, asset, *id);
					if let Some(res) = &handles
						&& let Some(res_css) = &res.css
					{
						commands
							.entity(entity)
							.insert(NodeStyleSheet::new(res_css.clone()));
					}
				}
			}
		}
	}
}

#[allow(clippy::needless_pass_by_value)]
fn html_ui_watch_load(
	mut commands: Commands,
	assets: Res<Assets<HtmlUiAsset>>,
	handles: Option<Res<HtmlCssUiResource>>,
	entity: Local<Option<Entity>>,
	q_roots: Query<(Entity, &HtmlUiRoot)>,
) {
	let Some(res) = handles else { return };
	let id_html = res.html.id();
	let opt_html = assets.get(id_html);
	let Some(asset_html) = opt_html else { return };
	if let Some(ent) = *entity {
		for (e, _) in q_roots.iter() {
			if e == ent {
				commands.entity(e).despawn();
			}
		}
	}
	let entity = spawn_html_ui(&mut commands, asset_html, id_html);
	if let Some(res_css) = &res.css {
		commands
			.entity(entity)
			.insert(NodeStyleSheet::new(res_css.clone()));
	}
}
