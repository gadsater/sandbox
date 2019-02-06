#include <8051.h>

void main() {
	unsigned char i;
	P1 = 0x00;
	while(1) {
		for (i=0; i<=255; i++) {
			P1++;
		}
	}
}
