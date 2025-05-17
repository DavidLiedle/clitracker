mod api;
mod models;
mod ui;

use ui::Ui;

fn main() {
    let ui = Ui::new();
    ui.init();
    ui.main_menu();
    ui.end();
}
