#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
using namespace std;

int main() {
	ull t, n, m;
	cin >> t;
	while (t--) {
		cin >> n >> m;
		ull count[n] = {0};
		ull flwr[n];
		for (ull i = 0; i < n; i++) {
			cin >> flwr[i];
		}
		sort(flwr, flwr+n);
		ull j = 0;
		for (ull i = 0; i < m; i++) {
			if ((count[j]+1)*flwr[j] <= (count[(j+1)%m]+1)*flwr[(j+1)%m]) {
				count[j]++;
			} else {
				j = (j + 1) % m;
				count[j]++;
			}
			//cout << j << " ";
		}
		//cout << endl;
		cout << count[j] * flwr[j] << endl;
	}
	return 0;
}

