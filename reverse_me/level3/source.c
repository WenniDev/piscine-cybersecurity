#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int wt() {
    return puts("********");
}

int nice() {
    return puts("nice");
}

int try() {
    return puts("try");
}

int but() {
    return puts("but");
}

int this() {
    return puts("this");
}

int it() {
    return puts("it");
}

int not() {
    return puts("not.");
}

int that() {
    return puts("that.");
}

int easy() {
    return puts("easy.");
}

void ___syscall_malloc() {
    puts("Nope.");
    exit(1);
}

int ____syscall_malloc() {
    return puts("Good job.");
}

int main() {
    printf("Please enter the key: ");

    char input[24];
    if (1 != scanf("%23s", input)) {
        ___syscall_malloc();
    }

    if (input[0] != '4') {
        ___syscall_malloc();
    }

    if (input[1] != '2') {
        ___syscall_malloc();
    }

    fflush(stdin);

    char result[9];
    memset(&result, 0, sizeof(result));

    result[0] = '*';

    int index_input = 2;
    int index_result = 1;

    while (true) {
        int var_4d_1 = 0;

        if (strlen(result) < 8) {
            if (index_input < strlen(input)) {
                break;
            }
        }

        char nptr[2];
        nptr[0] = input[index_input];
        nptr[1] = input[index_input + 1];

        result[index_result] = atoi(nptr);

        index_input += 3;
        index_result += 1;
    }

    result[index_result] = 0;

    switch (strcmp(result, "********")) {
        case 0: {
            ____syscall_malloc();
            return 0;
            break;
        }

        case 1:
        case 2:
        case 3:
        case 4:
        case 5:
        case 0x73:
        case 0xfffffffe:
        case 0xffffffff:
            ___syscall_malloc();
    }
    ___syscall_malloc();
    return 0;
}
