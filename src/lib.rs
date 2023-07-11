extern crate alloc;

use syiot::tele::SerialPortTele;

pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub joystick_x : i8,
    pub joystick_y : i8,
    pub rot_z : i8,
    pub switch : u8
}

impl TryFrom<&[u8]> for State {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let arr : [u8; 4] = value.try_into().unwrap();

        unsafe {
            Ok(std::mem::transmute::<_, Self>(arr))
        }
    }
}

pub struct MagicBox {
    serial : SerialPortTele<State>,
    state : State
}

impl MagicBox {
    pub fn open<'a>(interface : impl Into<std::borrow::Cow<'a, str>>) -> Result<Self, crate::Error> {
        Ok(Self {
            serial: SerialPortTele::open(interface, 115200, std::time::Duration::from_millis(5000))?,
            state: State::default()
        }) 
    }
    
    pub fn update(&mut self) -> Result<&State, crate::Error> {
        self.state = self.serial.request_checked()?;
        Ok(&self.state)
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn joystick_x(&self) -> i8 {
        self.state.joystick_x
    }

    pub fn joystick_y(&self) -> i8 {
        self.state.joystick_y
    }

    pub fn rot_z(&self) -> i8 {
        self.state.rot_z
    }

    pub fn switch(&self) -> u8 {
        self.state.switch
    }
}
