#include <malloc.h>

enum Action {
    WAIT,
    EXIT,
};

typedef struct {
    long id;
    void *data;
} df;

int request(df *fragment);
void submit(df fragment);
void spawn(void *entry, void *ctx);
df wait(df fragment);

df df_create();

static int cast_int(df fragment) {
    return *((int *)fragment.data);
}

static void *alloc(size_t size) {
    return calloc(1, size);
}

static void dealloc(void *memory) {
    if (memory) {
        free(memory);
    }
}
