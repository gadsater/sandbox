#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
using namespace std;

ull s[1000001];  
void sieveOfEratosthenes(ull N, ull s[]) 
{ 
    vector <bool> prime(N+1, false); 
  
    for (ull i=2; i<=N; i+=2) 
        s[i] = 2; 
		
		ull sqn = sqrt(N);
    for (ull i=3; i<=N; i+=2) 
    { 
        if (prime[i] == false) 
        { 
            s[i] = i; 
  
            for (int j=i; j*i<=N; j+=2) 
            { 
                if (prime[i*j] == false) 
                { 
                    prime[i*j] = true; 
  
                    s[i*j] = i; 
                } 
            } 
        } 
    } 
} 
  

int main() {
	sieveOfEratosthenes(1000000, s);
	s[0] = 1;
	s[1] = 1;
	//for (ull i = 100000; i < 1000000; i++) {
	//	cout << s[i] << " ";
	//}
	//cout << endl;
	ull t, n, m, temp, sum;
	cin >> t;
	while (t--) {
		sum = 0;
		cin >> n >> m;
		priority_queue <ull> que;
		for (ull i = 0; i < n; i++) {
			cin >> temp;
			que.push(temp);
		}
		for (ull i = 0; i < m; i++) {
			temp = que.top();
			//cout << temp << endl;
			que.pop();
			sum += temp;
			temp /= s[temp];
			que.push(temp);
		}
		cout << sum << endl;
	}
	return 0;
}

