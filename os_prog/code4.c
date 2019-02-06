#include <pthread.h>
#include <sys/types.h>
#include <stdio.h>
#include <stdlib.h>

#define ARR_SIZE 5

struct varg_input{
	int size;
	int *arr;
};


void *print_hello() {
	printf("Hello World\n");
}

void *print_num(void *num) {
	printf("number: %d\n", (int *)num);
}

void *print_sum(void *vargs) {
	struct varg_input *inp_struct = (struct varg_input *) vargs;
	int sum = 0;
	for (int i = 0; i < inp_struct->size; i++) {
		sum += inp_struct->arr[i];
	}
	printf("sum: %d\n", sum);
}

int main() {
	int dstate;

	pthread_t thr1;
	pthread_t thr2;
	pthread_t thr3;
	struct varg_input vargs;

	vargs.size = ARR_SIZE;
	vargs.arr = (int *) malloc(sizeof(int) * ARR_SIZE);
	for (int i = 0; i < ARR_SIZE; i++) {
		vargs.arr[i] = i + 1;
	}

	pthread_attr_t tattr;
	pthread_attr_init(&tattr);
	pthread_attr_setdetachstate(&tattr, PTHREAD_CREATE_JOINABLE);


	pthread_create(&thr1, NULL, print_hello, NULL);
	pthread_create(&thr2, NULL, print_num, (void *)3);
	pthread_create(&thr3, &tattr, print_sum, (void *)&vargs);
	pthread_attr_getdetachstate(&tattr, &dstate);
	printf("prev dstate: %d\n", dstate);

	pthread_join(thr1, NULL);
	pthread_join(thr2, NULL);
	pthread_join(thr3, NULL);

	printf("next dstate: %d\n", dstate);

	return 0;
}
