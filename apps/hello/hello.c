#include <stdint.h>
#include "../debug_exit.h"
#include "../uart.h"

void _start()
{
    prints("Hello, World!\n");
    debug_exit(0);
}
