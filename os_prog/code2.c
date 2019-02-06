#include<stdio.h>
#include<fcntl.h>
#include<unistd.h>
#include<sys/types.h>

int main(int argc, char **argv) {
	char str1[20];
	char str2[20];
	int fd1 = open("text.txt", O_CREAT | O_RDWR);
	int fd2 = open("text.txt", O_CREAT | O_RDWR);
	lseek(fd2, 1, SEEK_SET);
	read(fd1, str1, 20);
	read(fd2, str2, 20);
	printf("%s\n", str1);
	printf("%s\n", str2);
	return 0;
}
