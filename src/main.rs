use freya::prelude::*;

use commands::build_commands_list;
use components::command_palette::CommandPalette;

mod commands;
mod components;

fn main() {
    // *Start* your app with a window and its root component
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(1280., 800.)
                .with_title("Glyphwork"),
        ),
    )
}

fn app() -> impl IntoElement {
    let mut command_palette_shown = use_state(|| false);
    let commands_list = build_commands_list();

    rect()
        .padding(15.)
        .expanded()
        .background(Color::BLACK)
        .child(
            label()
                .text("Hello, World!")
                .font_size(24.)
                .font_weight(600)
                .color(Color::WHITE),
        )
        .maybe_child(command_palette_shown.read().then(|| CommandPalette {
            commands_list,
            on_close: EventHandler::new(move |_| {
                command_palette_shown.toggle();
            }),
        }))
        .on_global_key_down(move |e: Event<KeyboardEventData>| {
            if e.key == Key::Character("k".to_string()) && e.modifiers.ctrl() {
                command_palette_shown.toggle();
            }
        })
}
