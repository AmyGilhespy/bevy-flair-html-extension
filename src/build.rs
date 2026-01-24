use bevy::{
	input_focus::AutoFocus, prelude::*, ui::auto_directional_navigation::AutoDirectionalNavigation,
};
use bevy_flair::prelude::*;

use crate::{
	asset::HtmlUiAsset,
	ast::{HtmlElement, HtmlNode, HtmlTag},
};

#[derive(Component)]
pub struct HtmlUiRoot {
	//pub handle: Handle<HtmlUiAsset>,
	pub id: AssetId<HtmlUiAsset>,
}

pub fn spawn_html_ui(
	commands: &mut Commands,
	asset: &HtmlUiAsset,
	id: AssetId<HtmlUiAsset>,
	style_sheet: NodeStyleSheet,
) -> Entity {
	let root_entity = commands
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

	for node in &asset.ast {
		spawn_node(commands, root_entity, node);
	}

	root_entity
}

fn spawn_node(commands: &mut Commands, parent: Entity, node: &HtmlNode) {
	match node {
		HtmlNode::Text(text) => {
			/*
			commands.entity(parent).with_children(|p| {
				p.spawn(Text::new(text.clone()));
			});
			*/
			commands.entity(parent).insert(Text::new(text.clone()));
		}

		HtmlNode::Element(HtmlElement {
			tag,
			name_id,
			classes,
			gap,
			autofocus,
			children,
		}) => {
			let mut entity = commands.spawn((Node::default(),));

			if let Some(name) = name_id {
				entity.insert(Name::new(name.clone()));
			}

			if *autofocus {
				entity.insert(AutoFocus);
			}

			if !classes.is_empty() {
				entity.insert(ClassList::new(classes.join(" ").as_str()));
			}

			#[allow(clippy::match_same_arms)]
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
					// Label nodes will get text children
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

			let id = entity.id();
			commands.entity(parent).add_child(id);

			for child in children {
				spawn_node(commands, id, child);
			}
		}
	}
}
