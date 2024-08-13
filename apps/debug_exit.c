#include <stdint.h>
#include "debug_exit.h"

void debug_exit(uint8_t exit_status)
{
    uint8_t *debug_exit = (uint8_t *)DEBUG_EXIT_BASE_ADDR;
    *debug_exit = exit_status;
}
