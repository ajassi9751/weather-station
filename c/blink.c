#include <wiringPi.h> // Include WiringPi library!
#include <stdio.h>
#include <unistd.h>

int main()
{
	blink();
}

blink () {
  // uses BCM numbering of the GPIOs and directly accesses the GPIO registers.
  wiringPiSetupGpio();

  // pin mode ..(INPUT, OUTPUT, PWM_OUTPUT, GPIO_CLOCK)
  // set pin 17 to input
  pinMode(17, OUTPUT);

  // pull up/down mode (PUD_OFF, PUD_UP, PUD_DOWN) => down
  // pullUpDnControl(17, PUD_DOWN);

  // get state of pin 17
  int value = digitalRead(17);

  if (HIGH == value)
  {
    // your code
    printf("Its high\n");
  }
  digitalWrite(17, LOW);
  while (true) {
    printf("Loop\n");
    digitalWrite(17, HIGH);	
    sleep(1);
    digitalWrite(17, LOW);
    sleep(1);
  }
}
