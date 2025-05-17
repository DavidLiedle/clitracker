use crate::api::Api;
use ncurses::*;

pub enum MenuItem {
    Projects,
    Stories,
    Update,
    Exit,
}

pub struct Ui {
    api: Api,
}

impl Ui {
    pub fn new() -> Self {
        Ui { api: Api::new() }
    }

    pub fn init(&self) {
        initscr();
        keypad(stdscr(), true);
        noecho();
    }

    pub fn end(&self) {
        endwin();
    }

    pub fn main_menu(&self) {
        // Stub: main menu loop
        self.print_menu();
    }

    fn print_menu(&self) {
        mvprintw(0, 0, "CLITracker");
        mvprintw(2, 0, "[1] View My Projects");
        mvprintw(3, 0, "[2] View Stories by Project");
        mvprintw(4, 0, "[3] Update Story Status");
        mvprintw(5, 0, "[4] Exit");
        refresh();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn create_ui() {
        env::set_var("PT_API_TOKEN", "test");
        let _ui = Ui::new();
    }
}
