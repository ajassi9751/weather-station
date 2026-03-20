#!/usr/bin/env python

# Import GPIO and time libraries
import RPi.GPIO as GPIO
import time

# Set up GPIO
GPIO.setmode(GPIO.BOARD)
GPIO.setup(12,GPIO.IN)

# Anamometer vane diameter (set to the value for your cup-to-cup in mm)
vane_diameter = float(125)

# Calculate vane circumference in metres
vane_circ = float (vane_diameter/1000)*3.1415

# Set an anamometer factor to account for inefficiency (value is a guess)
afactor = float(2.5)

# Start measuring wind speed and let us know things are happening!
print('Measuring wind speed...')

# Define variables rotations and trigger (trigger = 1 if sensor triggered)
rotations = float(0)
trigger = 0

# Define variable endtime to be current time in seconds plus 10 seconds
endtime = time.time() + 10

# Get initial state of sensor
sensorstart = GPIO.input(12)

# Measurement loop to run for 10 seconds
while time.time() < endtime:
	if GPIO.input(12)==1 and trigger==0:
		rotations = rotations + 1
		trigger=1
	if GPIO.input(12)==0:
		trigger = 0
	# We seem to need to a little delay to make things work reliably...
	time.sleep(0.001)

# Loop has now finished. But if sensor triggered at start and did not move,
# rotations value will be 1, which is probably wrong, so . . .
if rotations==1 and sensorstart==1:
	rotations = 0

# Calculate stuff!
rots_per_second = float(rotations/10)
windspeed = float((rots_per_second)*vane_circ*afactor)

# Print results with decent formatting! :)
print('{:.0f} rotations = {:.2f} rotations/second'.format(rotations, rotations/10))
print('Windspeed is {:.2f} m/s = {:.2f} mph'.format(windspeed, windspeed*2.237))

# cleanup the GPIO before finishing :)
GPIO.cleanup()
