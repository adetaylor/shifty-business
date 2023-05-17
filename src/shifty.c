#include <stdint.h>

uint64_t guarded_rshift(uint64_t x, unsigned r) {
    return r > 63 ? 0 : x >> r;
}
