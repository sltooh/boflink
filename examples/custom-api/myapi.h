#ifndef MYAPI_H
#define MYAPI_H

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <stdint.h>

__declspec(dllimport) int MyApiVersion(void);
__declspec(dllimport) void MyApiPrintf(const char *format, ...);
__declspec(dllimport) void *MyApiAlloc(size_t size);
__declspec(dllimport) void MyApiFree(void *ptr);

#ifdef __cplusplus
};
#endif // __cplusplus

#endif // MYAPI_H
