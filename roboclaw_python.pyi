from .roboclaw_python import *

class Motor:
    M1 = 1
    M2 = 2

class RoboClaw:  
    """
    Class to control the roboclaw
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
    def read_encoder(self, motor: Motor, address: int = None) -> int:
        """
        Reads and returns the encoder value of the specified motor
        """
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