#include <pthread.h>
#include <sys/types.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SIZE 10
#define MIN_SIZE 0
#define BUF_SIZE 100

struct shared_resource {
	int size;
	char resource_arr[MAX_SIZE][BUF_SIZE];
} shr;

int flag;

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t prod_cond = PTHREAD_COND_INITIALIZER;
pthread_cond_t cons_cond = PTHREAD_COND_INITIALIZER;

void *consumer() {
	int i = 0;
	while(1) {
		pthread_mutex_lock(&mut);
		if(flag == 1 || shr.size == MIN_SIZE)
			pthread_cond_wait(&cons_cond, &mut);
		i = (i + 1) % MAX_SIZE;
		printf("consumed: %s\n", shr.resource_arr[i]);
		memset(shr.resource_arr[i], 0, BUF_SIZE);
		shr.size--;
		pthread_cond_signal(&prod_cond);
		pthread_mutex_unlock(&mut); 
		
	}	
}

void *producer() {
	int i = 0;
	while(1) {
		pthread_mutex_lock(&mut);
		if (shr.size == MAX_SIZE)
			pthread_cond_wait(&prod_cond, &mut);
		i = (i + 1) % MAX_SIZE;
		flag = 1;
		printf("Enter producer resource: ");
		scanf("%s", shr.resource_arr[i]);
		shr.size++;
		flag = 0;
		pthread_cond_signal(&cons_cond);
		pthread_mutex_unlock(&mut);
	}
}

int main () {
	pthread_t cons;
	pthread_t prod;
	shr.size = 0;
	pthread_create(&cons, NULL, consumer, NULL);
	pthread_create(&prod, NULL, producer, NULL);

	pthread_join(prod, NULL);
	pthread_join(cons, NULL);

	return 0;
}
