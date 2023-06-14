use core::time::Duration;

use magicbox::MagicBox;

use mouse_rs::Mouse;

fn main() -> Result<(), magicbox::Error> {
    // let interface = 

    let mut mbox = MagicBox::connect("COM5")?;

    let mouse = Mouse::new();
    let mut trig = false;
    let mut cnt = 0;

    mbox.listen();

    loop {
        let mouse_pos = mouse.get_position()?;
        
        mouse.move_to(mouse_pos.x + (mbox.joystick_x() as f32 / 10.0) as i32, mouse_pos.y - (mbox.joystick_y() as f32 / 10.0) as i32)?;

        // if mbox.switch() {
        //     if !trig {
        //         trig = true;
        //         mouse.click(&mouse_rs::types::keys::Keys::LEFT)?;
        //     }
        // } else {
        //     trig = false;
        // }

        std::thread::sleep(Duration::from_millis(10));

        cnt += 1; 

        if cnt == 10 {
            cnt = 0; 

            let delta = mbox.rot_z();

            if delta != 0 {
                // mouse.scroll(delta as i32)?;
            }
        }
    }
}