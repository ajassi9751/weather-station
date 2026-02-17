#include<unistd.h>
#include<stdio.h>
#include<wiringPi.h>

#define channel 17

main () {
	printf("Start\n");
	int data[100];
	int j = 0;
	setupWiringPiGpio();
	sleep(1);
	pinMode(channel, OUTPUT);
	digitalWrite(channel, LOW);
	delayMicroseconds(20000);
	digitalWrite(channel, HIGH);
	// pinMode(channel, INPUT);
	while (digitalRead(channel)==LOW) {
		continue;
	}
	while (digitalRead(channel)==HIGH) {
		continue;
	}
	while (j<40) {
		while () {

		}
	}
}

while j < 40:
	k = 0
	while GPIO.input(channel) == GPIO.LOW:
		continue

	while GPIO.input(channel) == GPIO.HIGH:
		k += 1
		if k > 100:
			break

	if k < 8:
		data.append(0)
	else:
		data.append(1)

	j += 1

print ("sensor is working.")
print (data)

humidity_bit = data[0:8]
humidity_point_bit = data[8:16]
temperature_bit = data[16:24]
temperature_point_bit = data[24:32]
check_bit = data[32:40]

humidity = 0
humidity_point = 0
temperature = 0
temperature_point = 0
check = 0

for i in range(8):
	humidity += humidity_bit[i] * 2 ** (7 - i)
	humidity_point += humidity_point_bit[i] * 2 ** (7 - i)
	temperature += temperature_bit[i] * 2 ** (7 - i)
	temperature_point += temperature_point_bit[i] * 2 ** (7 - i)
	check += check_bit[i] * 2 ** (7 - i)

tmp = humidity + humidity_point + temperature + temperature_point

if check == tmp:
	print ("temperature : ", temperature, ", humidity : " , humidity)
else:
	print ("wrong")
	print ("temperature : ", temperature, ", humidity : " , humidity, " check : ", check, " tmp : ", tmp)

GPIO.cleanup()
