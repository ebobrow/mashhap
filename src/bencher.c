#include <stdio.h>
#include <string.h>
#include <time.h>

#include "bencher.h"

Bencher bencher;

void initBencher() {
    initTable(&bencher.strings);
}

void freeBencher() {
    freeTable(&bencher.strings);
}

void runBench() {
    Table table;
    initTable(&table);

    // Insert increasingly long strings of "a"
    float startTime = (float)clock()/CLOCKS_PER_SEC;

    char str1[100000], str2[2];
    strcpy(str1, "a");
    for (int i = 0; i < 99999; i++) {
        strcpy(str2, "a");
        strcat(str1, str2);
        tableSet(&table, copyString(str1, i + 2), i);
    }

    float endTime = (float)clock()/CLOCKS_PER_SEC;

    float timeElapsed = endTime - startTime;
    printf("%f\n", timeElapsed);

    freeTable(&table);
}
