#!/usr/bin/env python

# Import all libraries we need!
from smbus import SMBus
from bme280 import BME280
import time
import datetime
from datetime import date
from openpyxl import load_workbook
import RPi.GPIO as GPIO

# Set up GPIO for wind speed measurement
GPIO.setmode(GPIO.BOARD)
GPIO.setup(12,GPIO.IN)

# Anamometer vane diameter (set to the value for your cup-to-cup in mm)
vane_diameter = float(106)

# Calculate vane circumference in metres
vane_circ = float (vane_diameter/1000)*3.1415

# Set an anamometer factor to account for inefficiency (value is a guess)
afactor = float(2.5)

# Initialise the BME280
bus = SMBus(1)
bme280 = BME280(i2c_dev=bus)

# Take first reading and disgard it to avoid garbage first row
temperature = bme280.get_temperature()
pressure = bme280.get_pressure()
humidity = bme280.get_humidity()
time.sleep(1)	

# Load the workbook and select the sheet
wb = load_workbook('/home/pi/Python_Code/weather.xlsx')
sheet = wb['Sheet1']

try:
	while True:
		
		# Read the sensor and get date and time
		temperature = round(bme280.get_temperature(),1)
		pressure = round(bme280.get_pressure(),1)
		humidity = round(bme280.get_humidity(),1)
		today = date.today()
		now = datetime.datetime.now().time()
		
		#Measure wind speed
		rotations = float(0)
		trigger = 0
		endtime = time.time() + 10
		sensorstart = GPIO.input(12)
		while time.time() < endtime:
			if GPIO.input(12)==1 and trigger==0:
				rotations = rotations + 1
				trigger=1
			if GPIO.input(12)==0:
				trigger = 0
			time.sleep(0.001)

		if rotations==1 and sensorstart==1:
			rotations = 0
		rots_per_second = float(rotations/10)
		windspeed = float((rots_per_second)*vane_circ*afactor)
		
		# Print reading and add data to the spreadsheet
		print('Adding this data to the spreadsheet:')
		print('{:.1f}*C {}hPa {}% {:.2f}m/s'.format(temperature, pressure, humidity, windspeed))
		row = (today, now, temperature, pressure, humidity, windspeed)
		sheet.append(row)
		
		#Save the workbook
		wb.save('/home/pi/Python_Code/weather.xlsx')

		# Wait for 9 minutes 50 seconds (590 seconds) -- wind measurement took 10 secs
		time.sleep(590)

finally:
	# Make sure the workbook is saved!
	wb.save('/home/pi/Python_Code/weather.xlsx')
	
	print('Goodbye!')