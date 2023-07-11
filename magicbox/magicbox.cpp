# include "magicbox.hpp"

# include "Arduino.h"

namespace magicbox {
    void setup() {
        pinMode(PIN_BUZZER, OUTPUT);

        for (uint8_t i = 0; i < (sizeof(PIN_LED_RGB) / sizeof(uint8_t)); i++) {
            pinMode(PIN_LED_RGB[i], OUTPUT);
        }

        for (uint8_t i = 0; i < (sizeof(PIN_JOYSTICK) / sizeof(uint8_t)); i++) {
            pinMode(PIN_JOYSTICK[i], INPUT);
        }

        pinMode(PIN_ROT_CLK, INPUT);
        pinMode(PIN_ROT_DT, INPUT);
        pinMode(PIN_ROT_SW, INPUT);

        ROT_STATE_LAST = digitalRead(PIN_ROT_CLK);
        ROT_STATE_CUR = 0; 
        ROT_DIR = CW;

        ROT_POS = 0;
        ROT_POS_LAST = 0;
    }

    State state() {
        return {
            joystick_x(),
            joystick_y(),
            rot_pos(),
            rot_switch()
        }; 
    }

    // RGB LED
        void set_red(uint8_t red) {
            analogWrite(PIN_LED_RGB[0], red);
        }

        void set_green(uint8_t green) {
            analogWrite(PIN_LED_RGB[1], green);
        }

        void set_blue(uint8_t blue) {
            analogWrite(PIN_LED_RGB[2], blue);
        }

        void set_rgb(uint8_t r, uint8_t g, uint8_t b) {
            set_red(r);
            set_green(g);
            set_blue(b);
        }
    // 

    // Buzzer 
        void buzz(uint32_t freq, unsigned long dur) {
            tone(PIN_BUZZER, freq, dur);
        }
    // 

    // Joystick
        int8_t joystick_x() {
            return (int8_t)map(analogRead(PIN_JOYSTICK[0]), 1023, 0, -100, 100); 
        }

        int8_t joystick_y() {
            return (int8_t)map(analogRead(PIN_JOYSTICK[1]), 1023, 0, -100, 100);
        }
    //

    // Rotary encoder 
        int rot_change() {
            int change = ROT_POS - ROT_POS_LAST;
            ROT_POS_LAST = ROT_POS;
            return change;
        }

        void update_rot() {
            ROT_STATE_CUR = digitalRead(PIN_ROT_CLK);

            if ((ROT_STATE_CUR != ROT_STATE_LAST) && (ROT_STATE_CUR == HIGH)) {
                if (digitalRead(PIN_ROT_DT) == ROT_STATE_CUR) {
                    ROT_POS++;
                    ROT_DIR = CCW;
                } else {
                    ROT_POS--;
                    ROT_DIR = CW;
                }
            }

            // Remember last CLK state
            ROT_STATE_LAST = ROT_STATE_CUR;
        }

        uint8_t rot_switch() {
            return digitalRead(PIN_ROT_SW);
        }

        int8_t rot_pos() {
            return (int8_t)map(magicbox::ROT_POS, -10, 10, -100, 100);
        }
    // 
}