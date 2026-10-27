use freya::prelude::*;

use crate::commands::Command;

#[derive(PartialEq)]
pub struct CommandPalette {
    pub commands_list: Vec<Command>,
    pub on_close: EventHandler<()>,
}

impl Component for CommandPalette {
    fn render(&self) -> impl IntoElement {
        let commands_list = self.commands_list.clone();
        let on_close = self.on_close.clone();
        let value = use_state(|| "".to_string());

        let filtered_commands = use_side_effect_value(move || {
            filter_commands(&value.read().to_string(), &commands_list)
        });

        rect()
            .position(Position::new_global())
            .expanded()
            .center()
            .child(
                rect()
                    .width(Size::px(200.))
                    .height(Size::px(80.))
                    .on_mouse_down(|e: Event<MouseEventData>| {
                        e.stop_propagation();
                    })
                    .child(
                        Input::new(value)
                            .placeholder("command here")
                            .auto_focus(true),
                    )
                    .children(filtered_commands.read().iter().map(|c| {
                        label()
                            .text(c.name.to_string())
                            .color(Color::WHITE)
                            .into_element()
                    })),
            )
            .background(Color::BLACK.with_a(127))
            .layer(50)
            .on_key_down({
                let on_close = on_close.clone();
                move |e: Event<KeyboardEventData>| match e.key {
                    Key::Named(NamedKey::Enter) => {
                        println!("{}", value.read());
                        on_close.call(());
                    }
                    Key::Named(NamedKey::Escape) => on_close.call(()),
                    _ => {}
                }
            })
            .on_mouse_down({
                move |_| {
                    on_close.call(());
                }
            })
    }
}

fn filter_commands(query: &str, commands_list: &[Command]) -> Vec<Command> {
    return commands_list
        .iter()
        .filter(|c| c.name.contains(query))
        .cloned()
        .collect();
}
