#ifndef WIND_SPEED
#define WIND_SPEED
#include "pi.h"
#if USE_PI == 1
#include <math.h>
#include <unistd.h>
#include <stdio.h>
#include <stdint.h>
#include <wiringPi.h>
void print_temp_humidity();
#endif
#endif