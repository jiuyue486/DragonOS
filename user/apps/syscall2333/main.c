#include <unistd.h>
#include <sys/syscall.h>
#include <errno.h>
#include <stdio.h>

int main(void) {
    errno = 0;
    long ret = syscall(2333);

    printf("syscall(2333) ret=%ld errno=%d\n", ret, errno);

    return 0;
}
