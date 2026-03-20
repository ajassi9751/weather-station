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
    double rotations;
    double rots_per_second;
    double windspeed;
    double windspeedMPH;
} Wind_data;
void get_wind_speed(Wind_data*);
#endif
#endif
