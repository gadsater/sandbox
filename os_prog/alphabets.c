#include<stdio.h>
#include<pthread.h>
#include<sys/types.h>

pthread_mutex_t m1 = PTHREAD_MUTEX_INITIALIZER;
pthread_mutex_t m2 = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cons = PTHREAD_COND_INITIALIZER;
pthread_cond_t vows = PTHREAD_COND_INITIALIZER;

char ch = 'a';


int isVowel(char a) {
	if (a == 'a' || a == 'e' || a == 'i' || a == 'o' || a == 'u') {
		return 1;
	}
	return 0;
}

void *print_vowels() {
	while (ch <= 'z') {
        pthread_mutex_lock(&m1);
		if (!isVowel(ch)) {
            pthread_cond_signal(&cons);
            pthread_cond_wait(&vows, &m1);
        }
        printf("vowels    : %c\n", ch);
        ch++;
		pthread_mutex_unlock(&m1);	
	}
	pthread_exit(0);
}

void *print_consonants() {
	while (ch <= 'z') {
		pthread_mutex_lock(&m2);
        if (isVowel(ch)) {
            pthread_cond_signal(&vows);
            pthread_cond_wait(&cons, &m2);
        }
        printf("consonants: %c\n", ch);
        ch++;
		pthread_mutex_unlock(&m2);
	}
	pthread_exit(0);
}

int main() {
	pthread_t conser, vowser;

	pthread_create(&vowser, NULL, print_vowels, NULL);
	pthread_create(&conser, NULL, print_consonants, NULL);	
	pthread_join(vowser, NULL);
	pthread_join(conser, NULL);
	printf("\n");
	return 0;
}
