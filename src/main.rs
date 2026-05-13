use std::io;
use tun_tap::{Iface, Mode};

fn main() -> io::Result<()>{
    let iface = Iface::new("tun0", Mode::Tun)?;

    let mut buffer = vec![0;1054];
    let nbytes = iface.recv(&mut buffer).unwrap();

    eprintln!("The bytes: {:?}", &buffer[..nbytes]);
    Ok(())
}
