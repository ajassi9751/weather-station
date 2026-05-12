#include "wind_speed.h"
#include "pi.h"
#if USE_PI == 1
#include <wiringPi.h>
#include <stdio.h>
#include <time.h>
#include <math.h>
#include "test.h"
#define CHANNEL 18

void print_wind_speed () {
	wiringPiSetupGpio();
	pinMode(CHANNEL, INPUT);
    const double vane_diameter = 125;
    const double vane_circ = (vane_diameter/1000)*M_PI;
    const double afactor = 2.5;
    printf("Measuring wind speed...\n");
    double rotations = 0;
    int trigger = 0;
    const long endtime = time(NULL) + 10;
    const int sensorstart = digitalRead(CHANNEL)==HIGH;
    while (time(NULL)<endtime) {
        if (digitalRead(CHANNEL)==HIGH && trigger==0) {
            rotations++;
            trigger = 1;
        }
        if (digitalRead(CHANNEL)==LOW)
            trigger = 0;
    	delayMicroseconds(1000);
    }
    if (rotations==1 && sensorstart==1)
        rotations = 0;
    double rots_per_second = ((double)rotations)/10;
    double windspeed = rots_per_second*vane_circ*afactor;
    printf("%f rotations = %f rotations/second\n", rotations, rots_per_second);
    printf("Windspeed is %f m/s = %f mph\n", windspeed, windspeed*2.237);
}

void get_wind_speed (Wind_data* data) {
	wiringPiSetupGpio();
	pinMode(CHANNEL, INPUT);
    const double vane_diameter = 125;
    const double vane_circ = (vane_diameter/1000)*M_PI;
    const double afactor = 2.5;
    // printf("Measuring wind speed...\n");
    double rotations = 0;
    int trigger = 0;
    const long endtime = time(NULL) + 10;
    const int sensorstart = digitalRead(CHANNEL)==HIGH;
    while (time(NULL)<endtime) {
        if (digitalRead(CHANNEL)==HIGH && trigger==0) {
            rotations++;
            trigger = 1;
        }
        if (digitalRead(CHANNEL)==LOW)
            trigger = 0;
    	delayMicroseconds(1000);
    }
    if (rotations==1 && sensorstart==1)
        rotations = 0;
    double rots_per_second = ((double)rotations)/10;
    double windspeed = rots_per_second*vane_circ*afactor;
    data->rotations = rotations;
    data->rots_per_second = rots_per_second;
    data->windspeed = windspeed;
    data->windspeedMPH = windspeed*2.237;
    // printf("%d rotations = %f rotations/second", rotations, rots_per_second);
    // printf("Windspeed is %f m/s = %f mph", windspeed, windspeed*2.237);
}

#if TEST == 1
int main () {
    print_wind_speed();
    // get_wind_speed();
}
#endif
#endif
