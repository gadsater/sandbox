#include <lpc214x.h>
int counter;
int temp;
int i;

void delay(void) {
	int j;
	int k;
	for (j = 0; j < 1000; j++)
		for	(k = 0; k < 1000; k++);
}

int main(void) {
	PINSEL0 = 0x00000000;
	PINSEL1 = 0x00000000;
	PINSEL2 = 0x00000000;
	IO0DIR =  0x00000000;
	IO1DIR =  0xFFFFFFFF;
	counter = 0;
	while(1) {
		if (!(IOPIN0&1)) {
			if (counter > 0) {
				counter--;
        temp = counter;
        for(i = 0; i < 4; i++) {
          if (temp & 1) {
            IOSET1 = 1 << (16+i);
          } else {
            IOCLR1 = 1 << (16+i);
          }
          temp = temp >> 1;
        }
			}
		}
		else {
			counter++;
			temp = counter;
			for(i = 0; i < 4; i++) {
				if (temp & 1) {
					IOSET1 = IOSET1 | 1 << (16+i);
				} else {
					IOCLR1 = IOCLR1 | 1 << (16+i);
				}
				temp = temp >> 1;
			}
		}
		delay();
	}
	return 0;
}
