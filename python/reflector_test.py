import RPi.GPIO as GPIO
import time

GPIO.setmode(GPIO.BOARD)
GPIO.setup(12, GPIO.IN)
try:
	while True:
		if GPIO.input(12)==1:
			print("White")
		else:
			print("Black")
		time.sleep(.001)
finally:
	GPIO.cleanup()
