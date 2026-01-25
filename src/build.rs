use bevy::{
	input_focus::AutoFocus, prelude::*, ui::auto_directional_navigation::AutoDirectionalNavigation,
};
use bevy_flair::prelude::*;

use crate::{
	asset::HtmlUiAsset,
	ast::{HtmlElement, HtmlNode, HtmlTag},
	error::HtmlUiError,
	resources::HtmlCssUiResource,
};

#[derive(Component)]
pub struct HtmlUiRoot {
	pub id: AssetId<HtmlUiAsset>,
}

#[allow(clippy::implicit_hasher)]
pub(crate) fn spawn_html_ui(
	world: &mut World,
	id: AssetId<HtmlUiAsset>,
) -> Result<Entity, HtmlUiError> {
	let root_entity: Entity;
	{
		let Some(res) = world.get_resource::<HtmlCssUiResource>() else {
			return Err(HtmlUiError::ResourceNotFound);
		};

		let mut style_sheet = NodeStyleSheet::Inherited;
		if let Some(res_css) = &res.css {
			style_sheet = NodeStyleSheet::new(res_css.clone());
		}

		root_entity = world
			.spawn((
				Node {
					position_type: PositionType::Absolute,
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					..default()
				},
				Pickable::IGNORE,
				HtmlUiRoot { id },
				style_sheet,
			))
			.id();
	}

	let Some(asset) = world.resource::<Assets<HtmlUiAsset>>().get(id) else {
		return Err(HtmlUiError::AssetNotFound);
	};

	let ast = asset.ast.clone();
	for node in &ast {
		let _ = spawn_node(world, root_entity, node);
	}

	Ok(root_entity)
}

#[allow(clippy::too_many_lines)]
fn spawn_node(world: &mut World, parent: Entity, node: &HtmlNode) -> Result<(), HtmlUiError> {
	match node {
		HtmlNode::Text(text) => {
			let text_entity = world.spawn(Text::new(text.clone())).id();

			world.entity_mut(parent).add_child(text_entity);

			Ok(())
		}

		HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			autofocus,
			callback,
			children,
		}) => {
			let mut entity: EntityWorldMut;
			{
				let Some(res) = world.get_resource::<HtmlCssUiResource>() else {
					return Err(HtmlUiError::ResourceNotFound);
				};

				let mut style_sheet = NodeStyleSheet::Inherited;
				if let Some(res_css) = &res.css {
					style_sheet = NodeStyleSheet::new(res_css.clone());
				}

				entity = world.spawn((Node::default(), style_sheet));
			}

			if let Some(name) = name_id {
				entity.insert(Name::new(name.clone()));
			}

			if *autofocus {
				entity.insert(AutoFocus);
			}

			if !classes.is_empty() {
				entity.insert(ClassList::new(classes.join(" ").as_str()));
			}

			match tag {
				HtmlTag::VBox => {
					entity.insert((
						Node {
							display: Display::Flex,
							flex_direction: FlexDirection::Column,
							row_gap: *gap,
							..default()
						},
						Pickable::IGNORE,
					));
				}
				HtmlTag::HBox => {
					entity.insert((
						Node {
							display: Display::Flex,
							flex_direction: FlexDirection::Row,
							column_gap: *gap,
							..default()
						},
						Pickable::IGNORE,
					));
				}
				HtmlTag::Node => {}
				HtmlTag::Button => {
					entity.insert((
						Button,
						AutoDirectionalNavigation::default(),
						Pickable {
							is_hoverable: true,
							should_block_lower: true,
						},
					));
				}
				HtmlTag::Label => {
					entity.insert(Pickable::IGNORE);
				}
				HtmlTag::Spacer => {
					entity.insert((
						Node {
							width: Val::Percent(100.0),
							height: Val::Percent(100.0),
							..default()
						},
						Pickable::IGNORE,
					));
				}
				HtmlTag::Ui => {
					entity.insert((
						Node {
							position_type: PositionType::Absolute,
							display: Display::Flex,
							flex_direction: FlexDirection::Column,
							row_gap: *gap,
							width: Val::Percent(100.0),
							height: Val::Percent(100.0),
							..default()
						},
						Pickable::IGNORE,
					));
				}
			}

			let entity_id = entity.id();

			world.entity_mut(parent).add_child(entity_id);

			for child in children {
				let _ = spawn_node(world, entity_id, child);
			}

			if let Some(cb_key) = callback {
				world.resource_scope(|world: &mut World, resource: Mut<HtmlCssUiResource>| {
					if let Some(cb) = resource.callbacks.get(cb_key) {
						return cb(world, entity_id);
					}
					Ok(())
				})
			} else {
				Ok(())
			}
		}
	}
}
