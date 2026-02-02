#include <stdio.h>
#include <string.h>

int main() {
    printf("Please enter key: ");
    char input[0x64];
    scanf("%s", input);

    if (strcmp(input, "__stack_check")) {
        printf("Nope.\n");
    } else {
        printf("Good job.\n");
    }
    return 0;
}
