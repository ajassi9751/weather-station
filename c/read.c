#include <wiringPi.h> // Include WiringPi library!
#include <stdio.h>
#include <unistd.h>

int main(void)
{
	// uses BCM numbering of the GPIOs and directly accesses the GPIO registers.
	wiringPiSetupGpio();

	// pin mode ..(INPUT, OUTPUT, PWM_OUTPUT, GPIO_CLOCK)
	// set pin 17 to input
	pinMode(17, OUTPUT);

	// pull up/down mode (PUD_OFF, PUD_UP, PUD_DOWN) => down
	// pullUpDnControl(17, PUD_DOWN);

	// get state of pin 17

	while (true) {
		// printf("Loop\n");
		int value = digitalRead(17);
		if (HIGH == value)
		{
			printf("Its high\n");
		}
		else {
			printf("Its low\n");
		}
		delayMicroseconds(1);
	// 	sleep(1);
	}
}
