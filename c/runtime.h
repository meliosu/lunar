#ifndef RUNTIME_H
#define RUNTIME_H

#include <stdint.h>

typedef enum {
    Continue,
    Wait,
    Exit,
} RET;

typedef struct {
    uint64_t value;
} ID;

typedef struct {
    ID id;
    void *value;
} DF;

typedef struct {
    ID id;
    void *block;
    void *context;
} CF;

int request(CF *self, DF *df);
int submit(CF *self, DF *df);
int spawn(CF *self, void *block, void *context);
int destroy(CF *self, DF *df);

#endif /* RUNTIME_H */
