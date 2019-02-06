#include<stdio.h>
#include<stdlib.h>
#include<sys/ipc.h>
#include<sys/types.h>
#include<sys/shm.h>
#include<string.h>
#define MAX_MSGCOUNT 5

struct shmds {
int msgcount;
char *msg[100];
}m;

int main(int argc, char **argv) {
	int shmid;
	char shmstr[MAX_MSGCOUNT][100];
	key_t key=1222;
	shmid=shmget(key,1024,0666);
	struct shmds *shmdata=(struct shmds *)shmat(shmid,NULL,0);
	if(argc==2 && !strcmp(argv[1],"clear")) {
  	shmdata->msgcount=0;
  	shmdata->msg[shmdata->msgcount]=NULL;
	} else {
		while(1) {
			while(shmdata->msgcount < MAX_MSGCOUNT) {
  			if(shmstr[shmdata->msgcount-1] != shmdata->msg[shmdata->msgcount] || shmdata->msgcount == 0) {
    			strcpy(shmstr[shmdata->msgcount], shmdata->msg[shmdata->msgcount]);
    			shmdata->msgcount++;
    			for(int i=0; i<shmdata->msgcount; i++) {
      			printf("%s ", shmstr[i]);
    			}	
    			printf("\n");
  			}
			}
		}
	}
	shmdt(shmdata);
	return 0;
}
