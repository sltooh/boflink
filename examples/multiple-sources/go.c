#include "beacon.h"

#include "other.h"

void go(void) {
    BeaconPrintf(CALLBACK_OUTPUT, "Hello world from the go() function");

    other_function();
}
