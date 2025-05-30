#include "myapi.h"

void go(void) {
    int version = MyApiVersion();
    MyApiPrintf("MyApiVersion: %d", version);

    int *value = MyApiAlloc(sizeof(int));

    *value = 123;
    MyApiPrintf("value: %d", *value);

    MyApiFree(value);
}
