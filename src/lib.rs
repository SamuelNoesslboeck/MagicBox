extern crate alloc;

use core::time::Duration;
use std::{ffi::OsStr, io::{Read, Write}, thread::JoinHandle, sync::Mutex};

use alloc::sync::Arc;
use serial::SerialPort;
#[cfg(windows)]
use serial::windows::COMPort;

pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, Debug, Default)]
pub struct BoxState {
    pub joystick_x : i8,
    pub joystick_y : i8,
    pub rot_z : i8,
    pub switch : bool
}

impl From<[u8; 4]> for BoxState {
    fn from(value: [u8; 4]) -> Self {
        unsafe {
            std::mem::transmute::<[u8; 4], Self>(value)
        }
    }
}

pub struct MagicBox {
    #[cfg(windows)]
    port : Arc<Mutex<COMPort>>,
    state : Arc<Mutex<BoxState>>,

    thr : Option<JoinHandle<()>>
}

impl MagicBox {
    pub fn connect<S : AsRef<OsStr> + ?Sized>(interface : &S) -> Result<Self, crate::Error> {
        let mut port = serial::open(interface)?;
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud115200)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(50))?;
    
        Ok(Self { 
            port: Arc::new(Mutex::new(port)), 
            state: Arc::new(Mutex::new(BoxState::default())),
            thr: None 
        })
    }

    pub fn listen(&mut self) {
        let port_mut = self.port.clone();
        let state_mut = self.state.clone();

        self.thr = Some(std::thread::spawn(move || {
            let mut buf = [0u8; 20];

            loop {
                let mut port = port_mut.lock().unwrap();
                if let Err(err) = port.read_exact(&mut buf) {
                    if !err.to_string().contains("timed out") {
                        panic!("Thread paniced! {}", err);
                    }
                };
                drop(port);

                let mut state = state_mut.lock().unwrap();
        
                for i in 0 .. 18 {
                    unsafe {
                        match buf[i] as char {
                            'x' => { 
                                state.joystick_x = std::mem::transmute(buf[i + 1]);
                                // X coord
                            }, 
                            'y' => {
                                // Y coord
                                state.joystick_y = std::mem::transmute(buf[i + 1]);
                            },
                            'z' => {
                                state.rot_z = std::mem::transmute(buf[i + 1]);
                            },
                            's' => {
                                state.switch = std::mem::transmute(buf[i + 1]);
                            },
                            _ => { }
                        }   
                    }
                }

                drop(state);
            }
        }));
    }

    pub fn joystick_x(&self) -> i8 {
        self.state.lock().unwrap().joystick_x
    }

    pub fn joystick_y(&self) -> i8 {
        self.state.lock().unwrap().joystick_y
    }

    pub fn rot_z(&self) -> i8 {
        self.state.lock().unwrap().rot_z
    }

    pub fn switch(&self) -> bool {
        self.state.lock().unwrap().switch
    }
}
