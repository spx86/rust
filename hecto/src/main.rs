#![warn(clippy::all, clippy::pedantic)]

use editor::editor::Editor;
mod editor;

fn main() {
    Editor::default().run();
}
