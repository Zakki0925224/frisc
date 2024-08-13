#include "uart.h"

void uart_putchar(char c)
{
    char *simple_uart = (char *)SIMPLE_UART_BASE_ADDR;
    *simple_uart = c;
}

void prints(const char *str)
{
    while (*str != '\0')
    {
        uart_putchar(*str);
        str++;
    }
}
