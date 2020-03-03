use io_arc::IoArc;
use std::io::{self, prelude::*};
use std::net::TcpListener;

#[allow(unused)]
fn run() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let (stream, _) = listener.accept()?;
    let mut stream = IoArc::new(stream);
    let mut stream = check_traits(stream);

    stream.write(b"hello world")?; // Write is implemented for Arc<TcpStream> directly
    Ok(())
}

fn check_traits<S>(s: S) -> S
where
    S: Clone + Read + Write,
{
    s
}
