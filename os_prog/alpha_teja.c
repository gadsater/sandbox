#include<stdio.h>
#include<pthread.h>
#include<unistd.h>
#include<sys/types.h>

pthread_t thd1;
pthread_t thd2;
pthread_mutex_t m1 = PTHREAD_MUTEX_INITIALIZER;
pthread_mutex_t m2 = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cons = PTHREAD_COND_INITIALIZER;
pthread_cond_t owl = PTHREAD_COND_INITIALIZER;
char j = 'a';

void *owls()
{

while(j <= 'z')
{  
pthread_mutex_lock(&m1);
if(j!= 'a'&& j!= 'e' &&j!= 'i' &&j!= 'o' &&j!= 'u') {
pthread_cond_signal(&cons);
pthread_cond_wait(&owl,&m1);
}
printf("vowel    : %c\n", j);
j++;
pthread_mutex_unlock(&m1);
//pthread_exit(0);
}
}

void *conss()
{
while(j <= 'z')
{
pthread_mutex_lock(&m2);
if(j== 'a' ||j== 'e'||j== 'i' ||j== 'o' ||j== 'u') { 
pthread_cond_signal(&owl);
pthread_cond_wait(&cons,&m2);
}
printf("consonant: %c\n", j);
j++;
pthread_mutex_unlock(&m2);
}


}
  
int main()
{
 pthread_create(&thd1,NULL,owls,NULL);
 pthread_create(&thd2,NULL,conss,NULL);
 pthread_join(thd1,NULL);
 pthread_join(thd2,NULL);
 pthread_exit(NULL);
}
