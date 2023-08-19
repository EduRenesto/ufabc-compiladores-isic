/* !!! auto-gerado por isic-back !!! */
#include <stdio.h>
#include <stdlib.h>

int main() {
    int a;
    int b;
    int c;
    int d;
    float f;
    f = 100.3f;
    a = 123;
    c = 666;
    printf("Digite um numero\n");
    scanf("%d", &b);
    if ((b < 10)) {
        printf("b menor 10\n");
    }
    else {
        printf("b maior ou igual 10\n");
    }
    if ((((a < b) && (c < d)) || (a < d))) {
        printf("blabla\n");
    }
    d = ((a * b) + (c * 10));
    printf("Hello world\n");
    printf("%d\n", a);
    printf("%d\n", b);
    printf("%d\n", c);
    int i;
    i = 0;
    while ((i < 10)) {
        printf("%d\n", i);
        i = (i + 1);
    }
    i = 0;
    do {
        printf("%d\n", i);
        i = (i + 1);
    } while ((i < 10));
}
