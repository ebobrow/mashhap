#ifndef clox_value_h
#define clox_value_h

#include <stdint.h>

typedef struct {
    int length;
    char* chars;
    uint32_t hash;
} String;

String* takeString(char* chars, int length);
String* copyString(const char* chars, int length);

#endif
