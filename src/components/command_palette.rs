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

        let mut highlighted_command_index = use_state(|| 0usize);
        let mut hovered_command_index: State<Option<usize>> = use_state(|| None);

        rect()
            .position(Position::new_global())
            .expanded()
            .cross_align(Alignment::Center)
            .child(
                rect()
                    .margin(Gaps::new(100., 0., 0., 0.))
                    .width(Size::px(200.))
                    .on_mouse_down(|e: Event<MouseEventData>| {
                        e.stop_propagation();
                    })
                    .spacing(12.)
                    .child(
                        Input::new(value)
                            .placeholder("command here")
                            .auto_focus(true)
                            .width(Size::fill())
                            .on_pre_key_down(|e: Event<KeyboardEventData>| {
                                // Stop arrow keys being captured by the input
                                !matches!(
                                    e.key,
                                    Key::Named(NamedKey::ArrowUp | NamedKey::ArrowDown)
                                )
                            })
                            .leading(label().text(">")),
                    )
                    .child(
                        rect().children(filtered_commands.read().iter().enumerate().map(
                            |(index, c)| {
                                rect()
                                    .color(Color::WHITE)
                                    .maybe(
                                        index == *highlighted_command_index.read()
                                            || hovered_command_index
                                                .read()
                                                .is_some_and(|i| i == index),
                                        |e| e.background(Color::WHITE.with_a(80)),
                                    )
                                    .width(Size::fill())
                                    .on_pointer_enter(move |_| {
                                        *hovered_command_index.write() = Some(index);
                                    })
                                    .on_pointer_leave(move |_| {
                                        *hovered_command_index.write() = None
                                    })
                                    .child(label().text(c.name.to_string()))
                                    .into_element()
                            },
                        )),
                    ),
            )
            .background(Color::BLACK.with_a(127))
            .layer(50)
            .on_global_key_down({
                let on_close = on_close.clone();
                move |e: Event<KeyboardEventData>| match e.key {
                    Key::Named(NamedKey::ArrowUp) => {
                        let prev_index = *highlighted_command_index.read();
                        *highlighted_command_index.write() =
                            subtract_with_modulo(prev_index, 1, filtered_commands.read().len());
                    }
                    Key::Named(NamedKey::ArrowDown) => {
                        let prev_index = *highlighted_command_index.read();
                        *highlighted_command_index.write() =
                            (prev_index + 1) % filtered_commands.read().len();
                    }
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

fn subtract_with_modulo(a: usize, b: usize, m: usize) -> usize {
    (a % m + m - b % m) % m
}
