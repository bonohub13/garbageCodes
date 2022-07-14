#include <stdio.h>

#define LEFT 0
#define RIGHT 1
#define BIT_INTS 32

typedef unsigned short int t_bool;

int bit_rotation(t_bool, int, unsigned int);

int main() {
    int i = 1;
    int j;

    printf("Initial value: %d\n", i);

    for (j=0; j<3; j++) {
        i = bit_rotation(LEFT, i, 1);

        printf("Iter: %d\n\tValue: %d\n", j+1, i);
    }

    return 0;
}

int bit_rotation(t_bool mode, int input_num, unsigned int bits_to_rotate) {
    if (mode == 0) {
        return (input_num << bits_to_rotate) | (input_num >> (BIT_INTS - bits_to_rotate));
    } else {
        return (input_num >> bits_to_rotate) | (input_num << (BIT_INTS - bits_to_rotate));
    }
}
