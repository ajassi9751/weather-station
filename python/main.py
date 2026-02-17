#!/usr/bin/env python3

import RPi.GPIO as GPIO
import time

LedPin = 17

GPIO.setmode(GPIO.BOARD)       # Use BCM numbering
GPIO.setup(LedPin, GPIO.OUT) # Set pin mode as output
GPIO.output(LedPin, GPIO.HIGH) # Set pin to high(+3.3V) to turn off the LED

try:
    while True:
        print('...led on')
        GPIO.output(LedPin, GPIO.LOW)  # LED on
        time.sleep(0.5)
        print('led off...')
        GPIO.output(LedPin, GPIO.HIGH) # LED off
        time.sleep(0.5)
except KeyboardInterrupt:  # When 'Ctrl+C' is pressed
    GPIO.output(LedPin, GPIO.HIGH)     # LED off
    GPIO.cleanup()                     # Release resources

