import dht11
import RPi.GPIO as GPIO
import time

# Set the GPIO pin
GPIO.setwarnings(False)
GPIO.setmode(GPIO.BCM)
pin = 17  # Change this to your GPIO pin
sensor = dht11.DHT11(pin=pin)

while True:
    result = sensor.read()
    if result.is_valid():
        print(f'Temperature: {result.temperature}°C  Humidity: {result.humidity}%')
    else:
        print('Failed to read data from the sensor')

    time.sleep(2)
