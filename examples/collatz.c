/* !!! auto-gerado por isic-back !!! */
#include <stdio.h>
#include <stdlib.h>

int main() {
    int n;
    printf("Digite um numero para iniciar a sequencia\n");
    scanf("%d", &n);
    while ((n > 1)) {
        if (((n % 2) == 0)) {
            n = (n / 2);
        }
        else {
            n = ((3 * n) + 1);
        }
        printf("%d\n", n);
    }
}
