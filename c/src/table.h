#ifndef clox_table_h
#define clox_table_h

#include <stdbool.h>
#include <stdint.h>

#include "value.h"

typedef struct {
    String* key;
    int value;
} Entry;

typedef struct {
    int count;
    int capacity;
    Entry* entries;
} Table;

void initTable(Table* table);
void freeTable(Table* table);
bool tableGet(Table* table, String* key, int* value);
bool tableSet(Table* table, String* key, int value);
bool tableDelete(Table* table, String* key);
void tableAddAll(Table* from, Table* to);
String* tableFindString(Table* table, const char* chars,
                           int length, uint32_t hash);

#endif
