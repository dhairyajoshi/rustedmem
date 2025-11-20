mod cmd;
mod store;
use cmd::handler::handle_command;
fn main() {
    loop {
        handle_command();
    }
}
