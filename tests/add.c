#include <malloc.h>
#include <stdio.h>

typedef struct {
    unsigned long id;
    void *value;
} Df;

void add(int a, int b, Df *df) {
    df->value = malloc(sizeof(int));
    *(int *)df->value = a + b;
}

void init(Df *df) {
    df->value = malloc(sizeof(int));
    *(int *)df->value = 10;
}

void print(Df *df) {
    printf("%d\n", *(int *)df->value);
}
