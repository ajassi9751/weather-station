from smbus2 import SMBus

# Replace 1 with 13 or 14 based on your i2cdetect output
bus_number = 14
device_address = 0x76

try:
    with SMBus(bus_number) as bus:
        # Read a byte to test connection
        bus.read_byte_data(device_address, 0x00)
        print(f"Sensor found on Bus {bus_number}!")
except Exception as e:
    print(f"Could not connect: {e}")
