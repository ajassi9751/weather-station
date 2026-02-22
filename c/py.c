#include <math.h>
#include<unistd.h>
#include<stdio.h>
#include<wiringPi.h>

#define CHANNEL 17
// This macro replicates the python y = x[0:1] syntax
// It also keeps list1i in its own scope so it doesn't cause redefinition issues
#define LIST_SEGMENT(list1, list2, range1, range2) {\
	int list1i = 0;\
		for (int i = range1; i < range2 - 1; i++) {\
			list1[list1i] = list2[i];\
			list1i++;\
		}\
}

int print_temp_humidity () {
	printf("Start\n");
	int data[100];
	int j = 0;
	int datai = 0;
	setupWiringPiGpio();
	sleep(1); // Maybe use delayMicroseconds
	pinMode(CHANNEL, OUTPUT);
	digitalWrite(CHANNEL, LOW);
	delayMicroseconds(20000);
	digitalWrite(CHANNEL, HIGH);
	// pinMode(channel, INPUT);
	while (digitalRead(CHANNEL)==LOW) {
		continue;
	}
	while (digitalRead(CHANNELL)==HIGH) {
		continue;
	}
	while (j<40) {
		int k = 0;
		while (digitalRead(CHANNEL)==LOW) {
			continue;
		}
		while (digitalRead(CHANNEL)==HIGH) {
			k++;
			if (k>100)
				break;
		}
		if (k<8) {
			data[datai] = 0;
			datai++;
		}
		else {
			data[datai] = 1;
			datai++;
		}
		j++;
	}
	printf("sensor is working.\n");
	for (int i = 0; i < datai-1; i++) {
		printf("%d\n", data[i]);
	}
	// Please no buffer overflow
	int humidity_bit[8];
	LIST_SEGMENT(humidity_bit, data, 0, 8);
	int humidity_point_bit[8];
	LIST_SEGMENT(humidity_point_bit, data, 8, 16);
	int temperature_bit[8];
	LIST_SEGMENT(temperature_bit, data, 16, 24);
	int temperature_point_bit[8];
	LIST_SEGMENT(temperature_point_bit, data, 24, 32);
	int check_bit[8];
	LIST_SEGMENT(check_bit, data, 32, 40);
	int humidity = 0;
	int humidity_point = 0;
	int temperature = 0;
	int temperature_point = 0;
	int check = 0;
	for (int i = 0; i < 8; i++) {
		humidity += pow(humidity_bit[i] * 2, 7 - i);
		humidity_point += pow(humidity_point_bit[i] * 2, 7 - i);
		temperature += pow(temperature_bit[i] * 2, 7 - i);
		temperature_point += pow(temperature_point_bit[i] * 2, 7 - i);
		check += pow(check_bit[i] * 2, 7 - i);
	}
	int tmp = humidity + humidity_point + temperature + temperature_point;
	if (check == tmp) {
		printf("temperature : %d, humidity : %d\n", temperature, humidity);
	}
	else {
		printf("wrong\n");
		printf("temperature : %d, humidity : %d check : %d tmp : %d\n", temperature, humidity, check, tmp);
	}
	// I dont think I need to clean up
}
