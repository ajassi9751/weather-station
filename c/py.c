#include <math.h>
#include <unistd.h>
#include <stdio.h>
#include <stdint.h>
#include <wiringPi.h>
#include "test.h"

#define CHANNEL 17
// This macro replicates the python y = x[0:1] syntax
// It also keeps list1i in its own scope so it doesn't cause redefinition issues
// Enter the first array [y], then the second [x], then range1 [0], then range2 [1]
// This will copy the defined range of elements from list2 into list1
// This will not copy the last element so something like 0,0 wont copy anything
// 0,1 will copy the first element
// 0,8 will copy 8 elements but only until index 7
#define LIST_SEGMENT(list1, list2, range1, range2) {\
	int list1i = 0;\
		for (int i = range1; i < range2 - 1; i++) {\
			list1[list1i] = list2[i];\
			list1i++;\
		}\
}

// The sensor seems to give us a 40 bit bianary sequence which we decode in 8 bit segments into 5 numbers
// The last number that we get should be equal all the others combined so it serves as a check
int print_temp_humidity () {
	printf("Start\n");
	int data[100];
	int j = 0;
	int datai = 0;
	setupWiringPiGpio();
	sleep(1); // Maybe use delayMicroseconds
	// Sets up the pin
	pinMode(CHANNEL, OUTPUT);
	// Seems to configure the pin by writing low and then high to the pin
	digitalWrite(CHANNEL, LOW);
	delayMicroseconds(20000);
	digitalWrite(CHANNEL, HIGH);
	// Possibly change the pin to input mode
	// pinMode(channel, INPUT);
	// Wait for the pin to first give a low then high signal
	while (digitalRead(CHANNEL)==LOW) {
		continue;
	}
	while (digitalRead(CHANNELL)==HIGH) {
		continue;
	}
	// Gather 40 points of data
	while (j<40) {
		int k = 0;
		// Ignore low signals
		while (digitalRead(CHANNEL)==LOW) {
			continue;
		}
		// Wait for 100 high signals or until there is a low signal
		while (digitalRead(CHANNEL)==HIGH) {
			k++;
			if (k>100)
				break;
		}
		// There was a low signal quickly after the long then write a 0 to the data
		if (k<8) {
			data[datai] = 0;
			datai++;
		} // If there was a low signal after a longer time or none at all then write a 1 to data
		else {
			data[datai] = 1;
			datai++;
		}
		j++;
	}
	printf("sensor is working.\n");
	// Print out the contents of data
	for (int i = 0; i < datai-1; i++) {
		// I think this works
		if (i == datai-2) {
			printf("%d\n", data[i]);
		}
		else {
			printf("%d, ", data[i]);
		}
	}
	// Please no buffer overflow
	// Create 5 smaller lists that all hold different segments of the data list
	int humidity_bit[8];
	LIST_SEGMENT(humidity_bit, data, 0, 8)
	int humidity_point_bit[8];
	LIST_SEGMENT(humidity_point_bit, data, 8, 16)
	int temperature_bit[8];
	LIST_SEGMENT(temperature_bit, data, 16, 24)
	int temperature_point_bit[8];
	LIST_SEGMENT(temperature_point_bit, data, 24, 32)
	int check_bit[8];
	LIST_SEGMENT(check_bit, data, 32, 40)
	// Create corresponding variables for the lists
	// These should be i8_t becuase the lists are exactly 8 bits so it makes sense
	// But that makes printing more annoying so ill change it later
	int humidity = 0;
	int humidity_point = 0;
	int temperature = 0;
	int temperature_point = 0;
	int check = 0;
	// Decodes a bianary number into decimal form
	for (int i = 0; i < 8; i++) {
		humidity += pow(humidity_bit[i] * 2, 7 - i);
		humidity_point += pow(humidity_point_bit[i] * 2, 7 - i);
		temperature += pow(temperature_bit[i] * 2, 7 - i);
		temperature_point += pow(temperature_point_bit[i] * 2, 7 - i);
		check += pow(check_bit[i] * 2, 7 - i);
	}
	int tmp = humidity + humidity_point + temperature + temperature_point;
	// The check number should be the same as the as of the others combined
	if (check == tmp) {
		printf("temperature : %d, humidity : %d\n", temperature, humidity);
	}
	else { // If it doesn't match, let the user know and print out the check and tmp as well
		printf("wrong\n");
		printf("temperature : %d, humidity : %d check : %d tmp : %d\n", temperature, humidity, check, tmp);
	}
	// I dont think I need to clean up wiringPi
}

#if TEST == 1
int main () {
	print_temp_humidity();
}
#endif
