# Hardware-Reference

The hardware reference gives an overview over the electrical components and connections used.

## Components used

- **MCU**: ESP32 (Type: NodeMCU32S)
- **LCD-Display**: LCD-1602A (I2C bus used)
- **LoRa-Module**: 

## ESP32 - Pin reference 

| ID                | Pin   | Description                                                       |
|-------------------|-------|-------------------------------------------------------------------|
|                   |       |                                                                   |
| **JoyStick**      |       |                                                                   | 
| JS_X              | 4     | Analog signal value for X direction of the Joystick (0-3.3V)      |
| JS_Y              | 34    | Analog signal value for Y direction of the Joystick (0-3.3V)      |
|                   |       |                                                                   |
| **Buttons**       |       |                                                                   |
| BT_A1             | 16    | First action button on the front                                  |
| BT_A2             | 17    | Second action button on the front                                 |
| BT_A3             | 25    | Third action button on the front                                  |
| BT_ULT            | 26    | Ultimate action button on the front                               |
|                   |       |                                                                   |
| **Rot-Encoder**   |       |                                                                   |
| RE_DT             | 39    | Data signal line of the rotary encoder                            |
| RE_CL             | 36    | Clock signal line of the rotary encoder                           |
| RE_SW             | 13    | Integrated rotary encoder button                                  |
|                   |       |                                                                   |
| **LoRa**          |       |                                                                   |
| LORA_SS           | 5     | >                                                                 |
| LORA_RST          | 14    | > LoRa Data connection                                            |
| LORA_D0           | 27    | >                                                                 |
|                   |       |                                                                   |
| **Misc.**         |       |                                                                   |
| BUZZER            | 2     | PWM Pin to play notes on the buzzer                               |
