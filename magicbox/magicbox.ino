# include "magicbox.hpp"
# include "telemetry_serial.hpp"

void setup() {
    Serial.begin(115200);

    magicbox::setup();
    magicbox::buzz(440, 200);   // Starting sound

    // magicbox::set_red(50);
    magicbox::set_green(1);
}

uint8_t buf [16];

void loop() {
    // Update the rotary encoder
    magicbox::update_rot();

    telemetry::update(&magicbox::state());
}