# MagicBox

The MagicBox is a WiFi/BlueTooth- and LoRa-able, programmable radio controller for a large range of applications. 

## Hardware

<p align="center">
    <img src="./documentation/controller_outline.jpg" width="50%">
</p>

Equipped with an ESP32 MCU, the controller can host quite challenging tasks and code. It's various input and output devices include:

- LCD Display
- Buzzer
- Rotary encoder
- Joystick
- 3 open and 1 shielded button

For more information, see the hardware reference [here](./documentation/hardware_reference.md).

## Software & Programming

The controller is fully programmable, meaning it can be adjusted to every application. Some basic APIs and libraries are provided:

- [magicontent](https://github.com/SamuelNoesslboeck/magicontent): Provides basic hardware bindings and APIs so you can start directly with tailoring the controller to your application
- [magicos](https://github.com/SamuelNoesslboeck/magicos): Some pre-made programs for general applications of the controller (LoRa-Mode, USB-Mode, ... )

The controller includes a simple OP-Code protocol called MBCP, which is generally transferred via LoRa. For more information, see [magicontent](https://github.com/SamuelNoesslboeck/magicontent).