#include<stdio.h>
#include<unistd.h>
#include<fcntl.h>
#include<sys/types.h>
#include<sys/wait.h>
#define MAX_DIGITS 4


int main() {
	int flag, temp = 0, stat;
	int pinp = 0, cval = 0;
	int status = fork();

	if(status == 0) {
		wait(&stat);
		int cinp = 0, i = 0;
		printf("Child's Input: ");
		scanf(" %d", &cinp);
		while(i < MAX_DIGITS) {
			if(flag != status) {
				temp = cinp % 10;
				cinp = cinp / 10;
				flag = status;
				i++;
				printf("%d %d\n",i, temp); 
			}
			raise(SIGCONT);
			wait(&stat);
		}
	} else {
		raise(SIGCONT);
		int i = 0;
		printf("Parent's Input: ");
		scanf(" %d", &pinp);
		while(i < MAX_DIGITS) {
				if(flag != status) {
					cval = cval * 10;
					cval += temp;
					flag = status;
					i++;
					printf("%d %d\n",i, temp); 
				}
				raise(SIGCONT);
				wait(&stat);
		}
		int res = pinp + cval;
		printf("output: %d", res);
	}
	return 0;
}
