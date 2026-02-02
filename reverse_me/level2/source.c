#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int ok() {
    return puts("Good job.");
}

void no() {
    puts("Nope.");
    exit(1);
}

int xd() {
    puts("Iii sapores crescit rei habetur disputo. An ab istud mo prius tanta error "
    "debet. Firma foret tes mea age capax sumne. Ex ex ipsas actum culpa neque ab saepe. "
    "Existenti et principia co immittere probandam imaginari re mo. Quapropter "
    "industriam ibi cui dissimilem cucurbitas progressus perciperem. Essendi ratione si "
    "habetur gi ignotas cognitu nusquam et.Sumpta vel uti ob");
    return puts("Author gi ex si im fallat istius. Refutent supposui qua sim "
    "nihilque. Me ob omni ideo gnum casu. Gi supersunt colligere inhaereat me sapientia "
    "is delaberer. Rom facillimam rem expe");
}

int main() {
    printf("Please enter key: ");

    char input[24];

    if (scanf("%23s", input) != 1) {
        no();
    }

    if (input[0] != '0') {
        no();
    }
    if (input[1] != '0') {
        no();
    }

    fflush(stdin);

    char result[9];
    memset(result, 0, 9);
    result[0] = 'd';

    int input_pos = 2;
    int result_pos = 1;

    while (strlen(result) < 8 && input_pos < strlen(input)) {
        char num_str[4];
        num_str[0] = input[input_pos];
        num_str[1] = input[input_pos + 1];
        num_str[2] = input[input_pos + 2];
        num_str[3] = '\0';

        result[result_pos] = (char)atoi(num_str);

        input_pos += 3;
        result_pos += 1;
    }

    result[result_pos] = '\0';

    if (strcmp(result, "delabere") != 0) {
        no();
    }

    ok();
    return 0;
}
