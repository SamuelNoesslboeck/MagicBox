# include "magicbox.hpp"

void setup() {
    Serial.begin(115200);

    magicbox::setup();
    magicbox::buzz(440, 200);

    // magicbox::set_red(50);
    magicbox::set_green(50);
}

void loop() {
    magicbox::update_rot();

    Serial.print('x');
    Serial.print((char)magicbox::joystick_x());
    Serial.print('y');
    Serial.print((char)magicbox::joystick_y());
    Serial.print('z');
    Serial.print((char)magicbox::rot_pos());
    Serial.print('s');
    Serial.print((char)magicbox::rot_switch());
}