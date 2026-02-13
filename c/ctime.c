#include <time.h>
#include <stdio.h>

char* get_c_time () {
    time_t time_now;
    time(&time_now);
    return ctime(&time_now);
}