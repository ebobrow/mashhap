#ifndef clox_vm_h
#define clox_vm_h

#include "table.h"
#include "value.h"

typedef struct {
    Table strings;
} VM;

extern VM vm;

void initVM();
void freeVM();

#endif
