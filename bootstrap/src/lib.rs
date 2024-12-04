use app::commands;
use soph::prelude::*;
use soph_console::{support::Command, Console};

pub struct App;

impl ApplicationTrait for App {
    type Service = Console;

    fn with_console() -> impl ServiceTrait {
        Console::new().register(Command::new::<Self, commands::Foo>())
    }
}
