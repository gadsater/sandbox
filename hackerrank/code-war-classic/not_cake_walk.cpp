#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
using namespace std;

int main() {
	ull t;
	cin >> t;
	while (t--) {
		ull a, b, k;
		cin >> a >> b >> k;
		ull x;
		if (a > k) {
			x = a + k - (a % k);
		} else {
			x = k;
		}
		if (x > b) {
		  x = ((a-(x-k))<(b-k))?(a-(x-k)):(b-k); 
		}
		cout << x << endl;
	}
	return 0;
}

