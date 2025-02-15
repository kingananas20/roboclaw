# RoboClaw Encoder Interface
This project provides an interface for controlling and reading encoder values from RoboClaw motor controllers. It allows you to set motor speeds, control motor movement, and read encoder values with proper handling of overflow, underflow, and direction.
Features

    Control Motors: Set speed for individual motors (M1, M2).
    Drive Both Motors: Control both motors at the same time with forward or backward movement.
    Turn Both Motors: Control turning by adjusting motor speeds in opposite directions.
    Read Encoder: Accurately read encoder values, adjusting for overflow and underflow.

## System Requirements
This library will only work on devices with the AArch64 architecture running linux/gnu.
Here's a list of example devices:
-   Raspberry PI 3/4/5 (Requires a 64-bit OS)
-   Raspberry PI Zero 2 W
-   NVIDIA Jetson Series

## Installation
    pip install roboclaw-python

## Usage
### Example in Python using the PyO3 bindings
    from roboclaw_python import RoboClaw, Motor
### Initialize RoboClaw
    roboclaw = RoboClaw(port_name="/dev/ttyUSB0", baud_rate=115200, timeout=20, retries=128, address=128)
### Set motor speed
    roboclaw.set_speed(Motor.M1, 100)
### Read encoder value
    encoder_value_m1 = roboclaw.read_encoder(Motor.M1)
    print(f"Motor M1 Encoder: {encoder_value_m1}")

## License
This project is licensed under the MIT License.