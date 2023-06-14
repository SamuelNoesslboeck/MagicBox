# pragma once

# include "Arduino.h"

# define CW true
# define CCW false

namespace magicbox {
    // Constants
        const uint8_t PIN_LED_RGB[3] = { 9, 10, 11 };

        const uint8_t PIN_BUZZER = 5;

        const uint8_t PIN_JOYSTICK[2] = { A1, A2 };

        const uint8_t PIN_ROT_CLK = 3;
        const uint8_t PIN_ROT_DT = 2;
        const uint8_t PIN_ROT_SW = 12;
    // 

    // Static variables
        static uint8_t ROT_STATE_CUR;
        static uint8_t ROT_STATE_LAST;

        static int32_t ROT_POS;
        static int32_t ROT_POS_LAST;
        static bool ROT_DIR;
    // 

    void setup();

    // RGB LED
        void set_red(uint8_t red);

        void set_green(uint8_t green);

        void set_blue(uint8_t blue);

        void set_rgb(uint8_t r, uint8_t g, uint8_t b);
    //

    // Buzzer
        void buzz(uint32_t freq, unsigned long dur = 0);
    // 

    // Joystick
        int8_t joystick_x();

        int8_t joystick_y();
    //

    // Rotary encoder 
        int rot_change();

        void update_rot();

        uint8_t rot_switch(); 

        int8_t rot_pos();
    //  
}