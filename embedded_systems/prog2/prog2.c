#include <8051.h>
#define port0 P0

void delay() {
	TMOD = 0x02;
	TH0 = 0x00;
	TL0 = 0x00;
	TR0 = 1;
	int i, j;
	for (i=0; i<15; i++) {
		for (j=0; j<255; j++) {
			while (TF0 != 1);
			TF0 = 0;
		}
	}
	TR0 = 0;
	TF0 = 0;
}

void main() {
	port0 = 0x00;

	while(1) {
		port0 = 0x55;
		delay();
		port0 = 0xff;
		delay();
	}
}
