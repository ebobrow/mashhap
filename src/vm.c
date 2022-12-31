#include "vm.h"

VM vm;

void initVM() {
    initTable(&vm.strings);
}

void freeVM() {
    freeTable(&vm.strings);
}
