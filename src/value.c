#include <string.h>

#include "memory.h"
#include "table.h"
#include "value.h"
#include "vm.h"

static String* allocateString(char* chars, int length,
                                 uint32_t hash) {
    String* string = (String*)reallocate(NULL, 0, sizeof(String));
    string->length = length;
    string->chars = chars;
    string->hash = hash;
    tableSet(&vm.strings, string, 0);
    return string;
}

static uint32_t hashString(const char* key, int length) {
    uint32_t hash = 2166136261u;
    for (int i = 0; i < length; i++) {
        hash ^= (uint8_t)key[i];
        hash *= 16777619;
    }
    return hash;
}

String* takeString(char* chars, int length) {
    uint32_t hash = hashString(chars, length);
    String* interned = tableFindString(&vm.strings, chars, length,
                                          hash);
    if (interned != NULL) {
        FREE_ARRAY(char, chars, length + 1);
        return interned;
    }

    return allocateString(chars, length, hash);
}

String* copyString(const char* chars, int length) {
    uint32_t hash = hashString(chars, length);
    String* interned = tableFindString(&vm.strings, chars, length,
                                          hash);
    if (interned != NULL) return interned;

    char* heapChars = ALLOCATE(char, length + 1);
    memcpy(heapChars, chars, length);
    heapChars[length] = '\0';
    return allocateString(heapChars, length, hash);
}
