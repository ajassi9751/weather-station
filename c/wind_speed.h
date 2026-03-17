#ifndef WIND_SPEED
#define WIND_SPEED
#include "pi.h"
#if USE_PI == 1
#include <wiringPi.h>
#include <stdio.h>
#include <time.h>
#include <math.h>
void print_wind_speed();
typedef struct {
    double rotations = 0;
    double rots_per_second = 0;
    double windspeed = 0;
    double windspeedMPH = 0;
} Wind_data;
Wind_data get_wind_speed();
#endif
#endif