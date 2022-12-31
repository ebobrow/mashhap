#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "common.h"
#include "table.h"
#include "value.h"
#include "vm.h"

int main() {
    initVM();

    Table table;
    initTable(&table);

    // benches (TODO)
    char* key = "bye";
    tableSet(&table, copyString(key, 3), 1);
    int ret;
    tableGet(&table, copyString(key, 3), &ret);
    printf("%d\n", ret);

    freeTable(&table);

    freeVM();
    return 0;
}
