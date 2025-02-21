from .roboclaw_python import *

class Motor:
    M1 = 1
    M2 = 2

class RoboClaw:  
    """
    Class to control one or multiple RoboClaws
    """

    def set_speed(self, motor: Motor, speed: int, address: int = None) -> bool: 
        """
        Sets the speed of a specified motor.

        ### Detailed Description 
        - motor: is either 1 or 2 
        - speed: positive to make the motor turn into the positive direction and negative for the other way around
        - address (optional): address of the roboclaw with the motor on (default to RoboClaw.new(address))
        """
    def drive(self, speed: int, address: int = None) -> bool:
        """
        Drives both motors in the same direction
        
        ### Detailed Description
        - speed: negative to drive forward, positive to drive backwards
        - address (optional): address of the roboclaw with the motors on (default to RoboClaw.new(address))
        """
    def turn(self, speed: int, address: int = None) -> bool:
        """
        Turn
        
        ### Detailed Description
        - speed: negative to turn left, positive to turn right
        - address (optional): address of the roboclaw with the motors on (default to RoboClaw.new(address))
        """

    #Encoder Commands
    def read_encoder(self, motor: Motor, address: int = None) -> int:
        """
        Reads and returns the encoder value of the specified motor
        """
    def read_encoder_speed(self, address: int = None) -> int:
        """
        Read encoder counter speed. Returned value is in pulses per second.
        RoboClaw keeps track of how many pulses received per second for both encoder channels.
        """
    def reset_encoders(self, address: int = None) -> bool:
        """
        Will reset both quadrature encoder counters to zero. This command applies to quadrature encoders only.
        """
    def set_encoder(self, motor: Motor, encoder_value: int, address: int = None) -> bool:
        """
        Set the value of the specified encoder register. Useful when homing. This command applies to quadrature encoders only.
        """
    def read_raw_speed(self, motor: Motor, address: int = None) -> int:
        """
        Read the pulses counted in the last 300th of a second. This is an unfiltered version of funciton read_encoder_speeds.
        This function can be used to make an independant PID routine. Value returned is in encoder counts per second.
        """
    def read_avg_speed(self, motor: Motor, address: int = None) -> int:
        """
        Read M1 or M2 average speed. Returns the speed in encoder counts per second.
        """

    #Advanced Commands
    def set_serial_timeout(self, timeout: int, address: int = None) -> bool:
        """
        Sets the serial communication timout in 100ms increments.
        When serial bytes are received in the time specified both motors will stop 
        automatically. Range is 0 to 25.5 seconds (0 to 255 in 100ms increments)
        """
    def read_serial_timeout(self, address: int = None) -> int:
        """
        Read the current serial timeout setting. Range is 0 to 255.
        """