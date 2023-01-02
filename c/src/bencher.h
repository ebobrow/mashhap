#ifndef clox_bencher_h
#define clox_bencher_h

#include "table.h"
#include "value.h"

typedef struct {
    Table strings;
} Bencher;

extern Bencher bencher;

void initBencher();
void freeBencher();
void runBench();

#endif
