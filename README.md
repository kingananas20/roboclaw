# RoboClaw Encoder Interface
This project provides an interface for controlling and reading encoder values from RoboClaw motor controllers. It allows you to set motor speeds, control motor movement, and read encoder values with proper handling of overflow, underflow, and direction.
Features

    Control Motors: Set speed for individual motors (M1, M2).
    Drive Both Motors: Control both motors at the same time with forward or backward movement.
    Turn Both Motors: Control turning by adjusting motor speeds in opposite directions.
    Read Encoder: Accurately read encoder values, adjusting for overflow and underflow.

## System Requirements
This library works on windows x86, linux x86 and linux aarch64 machines.
Linux supports glibc 2.34+ or musl 1.2+.
Python version needs to be 3.7 or higher.

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

## Currently supported
These are the currently supported functions. The number shows which RoboClaw commands it uses.
You can see a more detailed description in the [Roboclaw User Manual](https://downloads.basicmicro.com/docs/roboclaw_user_manual.pdf)

    set_speed()                 0 / 1 / 4 / 5
    drive()                     8 / 9
    turn()                      10 / 11

    //Encoders
    read_encoder()              16 / 17
    read_encoder_speed()        18 / 19
    reset_encoders()            20
    set_encoder()               22 / 23
    read_raw_speed()            30 / 31
    read_avg_speed()            108
    read_speed_error()          111
    read_position_error()       114

    //Advanced motor controls
    set_velocity_pid()          28 / 29
    set_speed_duty()            32 / 33
    drive_duty()                34

    //Advanced Commands
    set_serial_timeout()        14
    read_serial_timeout()       15

## External links
-   [GitHub Repository](https://github.com/kingananas20/roboclaw)
-   [Basic Micro](https://www.basicmicro.com/)
-   [Roboclaw User Manual](https://downloads.basicmicro.com/docs/roboclaw_user_manual.pdf)

## License
This project is licensed under the MIT License.

This is an unofficial roboclaw driver for python. I'm not affiliated with Basic Micro.