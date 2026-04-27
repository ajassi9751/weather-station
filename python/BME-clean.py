# Import modules for time and to access sensor
from smbus import SMBus
from bme280 import BME280

# Initialise the BME280
bus = SMBus(1)
bme280 = BME280(i2c_dev=bus)

# Get data
temperature = bme280.get_temperature()
pressure = bme280.get_pressure()
humidity = bme280.get_humidity()
# Print
print('{:05.2f}\n{:05.2f}\n{:05.2f}'.format(temperature, humidity, pressure))