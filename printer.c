#include<stdio.h>

extern void print_float(float f) {
    printf("%f\n", f);
}

extern void print_int(int i) {
    printf("%d\n", i);
}

void program();

int main()
{
    print_float(3.14159);
    program();
    return 0;
}
