use freya::prelude::*;

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
    rect().padding(15.).child(
        label()
            .text("Hello, World!")
            .font_size(24.)
            .font_weight(600),
    )
}
