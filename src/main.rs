mod alpm;
mod view;

use ::alpm::Alpm;
use iced::Task;

use crate::alpm::AlpmPkg;

#[derive(Debug)]
struct AppState {
    alpm: Alpm,
    start_pkgs: Vec<AlpmPkg>,
    pkg_selected: AlpmPkg,
}

#[derive(Debug, Clone)]
enum Message {
    Init,
    PkgSelected(AlpmPkg),
}

fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view).run()
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                alpm: Alpm::new("/", "/var/lib/pacman").unwrap(),
                start_pkgs: vec![],
                pkg_selected: AlpmPkg::default(),
            },
            Task::batch(vec![Task::done(Message::Init)]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init => {
                if let Ok(pkgs) = self.sync_alpm_dbs() {
                    self.start_pkgs = pkgs;
                }
                Task::none()
            }
            Message::PkgSelected(pkg_name) => {
                println!("{:?}", pkg_name);
                self.pkg_selected = pkg_name;
                Task::none()
            }
        }
    }
}
