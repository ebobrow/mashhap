#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "common.h"
#include "table.h"
#include "value.h"
#include "bencher.h"

int main() {
    initBencher();

    // benches (TODO)
    /* tableSet(&table, copyString("bye", 3), 2); */
    /* int ret; */
    /* tableGet(&table, copyString("bye", 3), &ret); */
    /* printf("%d\n", ret); */
    runBench();

    freeBencher();
    return 0;
}
