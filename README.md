# Bevy Flair HTML Extension
Bevy Flair HTML Extension is an extension crate for bevy_flair which adds basic HTML-like syntax with hot-reloading.

# Important Notes

1. This crate will remain in a very unstable state until the -alpha semver tag is removed.  Use with caution.
2. Documentation is coming soon.  Until then, this README contains the basic usage.
3. The term "HTML" as used in this crate is referring to an HTML-like DSL, not true standards-compliant HTML.

# HTML-like Syntax

Bevy Flair HTML Extension uses a non-standard HTML-like DSL for UI layout, and is meant to be used in conjunction with [bevy_flair](https://crates.io/crates/bevy_flair).

To be clear, the term "HTML" as used in the name of this crate and in this README is referring to an HTML-like DSL, not true standards-compliant HTML.  The purpose is to drastically simplify the process of creating and tweaking a UI, not to provide a 1-to-1 compatibility layer with the HTML used on the web.

# Syntax

Supported tags include:

- `<ui>` ... `</ui>`
	- The `<ui>` tag denotes the entire UI.
- `<vbox>` ... `</vbox>`
	- The `<vbox>` tag denotes a vertical box layout.
- `<hbox>` ... `</hbox>`
	- The `<hbox>` tag denotes a horizontal box layout.
- `<label>` ... `</label>`
	- The `<label>` tag inserts a `Text` element with the provided text.
- `<button>` ... `</button>`
	- The `<button>` tag inserts a `Button` element.
- `<spacer>` ... `</spacer>`
	- The `<spacer>` tag inserts a `Node` that expands in size as much as it can.

You can add `class="example another-example"` to add classes to a tag, just like in real HTML.

You can also add `gap="12.3px"` to any `vbox` or `hbox` to set the gap.  Supported units are `px`, `%`, `vw`, `vh`, `vmax`, and `vmin`, as well as the keyword `auto`.

# Usage Example

Here is an example HTML document from one of my projects:

```html
<ui class="title-screen">
	<hbox class="menu-area">
		<vbox class="menu-list" gap="15px">
			<spacer></spacer>
			<button class="button continue">
				<vbox>
					<spacer></spacer>
					<label class="label">Continue</label>
					<spacer></spacer>
				</vbox>
			</button>
			<button class="button load-game">
				<vbox>
					<spacer></spacer>
					<label class="label">Load Game</label>
					<spacer></spacer>
				</vbox>
			</button>
			<button class="button quit">
				<vbox>
					<spacer></spacer>
					<label class="label">Quit</label>
					<spacer></spacer>
				</vbox>
			</button>
		</vbox>
		<spacer></spacer>
	</hbox>
</ui>
```

You can use this Rust code to spawn the UI:

```rust
	commands.insert_resource(HtmlCssUiResource {
		html: asset_server.load("html/title_screen/title_screen.html"),
		css: Some(asset_server.load("css/title_screen/title_screen.css")),
	});
```

If you want to detect a button press, a simple way is to give your button a class and use bevy_flair's APIs to query for it:

```rust
fn title_menu_interaction(
	mut q_interaction: Query<(&Interaction, &ClassList), Changed<Interaction>>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	#[allow(clippy::explicit_iter_loop)]
	for (interaction, class_list) in q_interaction.iter_mut() {
		if let Interaction::Pressed = *interaction {
			if class_list.contains("button") {
				if class_list.contains("continue") {
					next_state.set(GameState::MainGame);
				}
			}
		}
	}
}
```

To remove the UI (such as on a state change), simply remove the resource:

```rust
	commands.remove_resource::<HtmlCssUiResource>();
```

Or, to change it, just overwrite the resource with the new one.

# Planned Features

1. Documentation
2. Unit Tests
3. Maybe add a better way to center text vertically in buttons.
4. Make spacer a self-closing tag.
5. Add i18n support for labels.

# License
Bevy Flair HTML Extension is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

    MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

# Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
