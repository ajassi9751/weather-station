#include "ctime.h"
#include "test.h"

#if TEST == 1
int main () {
	char* time = get_c_time();
	printf("%s", time);
}
#endif
