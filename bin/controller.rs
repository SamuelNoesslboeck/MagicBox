use clap::{arg, command};

use magicbox::MagicBox;
use syiot::remote::ControlClient;

fn main() -> Result<(), magicbox::Error> {
    let matches = command!()
        .arg(arg!([serial] "Name of the serial port the MagicBox is connected to"))
        .arg(arg!([addr] "Address to connect to"))
        .get_matches();

    let serial = matches.get_one::<String>("serial").ok_or("A vaild serial port must be provided!")?;
    let addr = matches.get_one::<String>("addr").ok_or("A vailid address to connect to must be provided!")?;

    let mut mbox = MagicBox::open(serial)?;
    let mut control = ControlClient::<magicbox::State>::new();

    control.connect(syiot::remote::Transport::FramedTcp, addr)?;

    loop {
        mbox.update()?;
        control.send(mbox.state())?;
    }
}