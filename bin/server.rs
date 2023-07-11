use clap::{arg, command};
use syiot::remote::{Control, ControlHandler};

struct Handler { }

impl ControlHandler<magicbox::State> for Handler {
    fn on_accept(&mut self) {
        println!(" => Accepted! "); 
    }

    fn on_msg(&mut self, msg : Result<magicbox::State, syiot::Error>) {
        if let Ok(state) = msg {
            dbg!(state);
        }
    }
}

fn main() -> Result<(), magicbox::Error> {
    let matches = command!()
        .arg(arg!([addr] "Address to host the server on"))
        .get_matches();

    let addr = matches.get_one::<String>("addr").ok_or("A vailid address to host the server must be provided!")?;
    let control = Control::<magicbox::State, _>::new(Handler { });

    control.listen(syiot::remote::Transport::FramedTcp, addr)?;

    control.run();

    Ok(())
}