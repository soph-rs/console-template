use clap::{ArgMatches, Command};
use soph::prelude::*;
use soph_console::traits::CommandTrait;

pub struct Foo;

#[async_trait]
impl CommandTrait for Foo {
    fn command() -> Command {
        Command::new("foo").about("foo command")
    }

    async fn handle<A: ApplicationTrait>(_: ArgMatches) -> Result<()> {
        println!("bar");

        Ok(())
    }
}
