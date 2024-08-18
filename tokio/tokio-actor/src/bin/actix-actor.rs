use std::io;

fn main() -> io::Result<()> {
    let system = actix::System::new();

    system.run()
}