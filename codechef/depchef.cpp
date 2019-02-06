#include <bits/stdc++.h>
using namespace std;

int chmod(int a, int b) {
	if(a % b >= 0) {
	 return a % b;
 	} else {
		return b + (a % b);
	}
}

int main() {
	int t;
	cin >> t;
	while (t--) {
		int n;
		cin >> n;
		int arra[n], arrb[n];
		for (int i = 0; i < n; i++) {
			cin >> arra[i];
		}
		for (int i = 0; i < n; i++) {
			cin >> arrb[i];
		}
		int res = -1;
		for (int i = 0; i < n; i++) {
			long long check = arrb[i] - arra[chmod((i + 1), n)] - arra[chmod((i - 1), n)];
			if (check > 0) {
				if (arrb[i] > res) {
					res = arrb[i];
				}
			}
		}
		cout << res << "\n";
	}
	return 0;
}
