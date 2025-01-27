use imgui::{Condition, Ui};

pub fn render(ui: &Ui, app_state: &mut super::AppState, running: &mut bool) {
    ui.window("Main Menu")
        .size([400.0, 200.0], Condition::Always)
        .position([440.0, 260.0], Condition::Always)
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .build(|| {
            ui.text("Welcome");
            ui.separator();
            if ui.button("Start") {
                *app_state = super::AppState::Content;
            }
            if ui.button("Exit") {
                *running = false;
            }
        });
}
