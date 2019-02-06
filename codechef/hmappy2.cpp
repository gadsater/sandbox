#include <bits/stdc++.h>
using namespace std;

int main() {
	int t, divab, divarb, lcmab;
	cin >> t;
	while (t--) {
		long long k, n, a, b;
		cin >> n >> a >> b >> k;
		lcmab = (a * b) / __gcd(a, b);
		divab = n / lcmab;
		divarb = (n / a) + (n / b) - (2 * divab);
		if (divarb < k) {
			cout << "Lose\n";
		} else {
			cout << "Win\n";
		}
	}
	return 0;
}
