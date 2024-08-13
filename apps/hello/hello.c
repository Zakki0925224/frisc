void _start()
{
    volatile char *addr = (volatile char *)0x123;
    const char *msg = "Hello, World!\n";

    while (*msg != '\0')
    {
        *addr = *msg;
        addr++;
        msg++;
    }
}
