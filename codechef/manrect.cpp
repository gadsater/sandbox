#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
#define MAX 1000000000
using namespace std;


int main() {
	ull t, xl, xu, yl, yu;
	cin >> t;
	while (t--) {
		ull q1, q2, q3, q4;
		cout << "Q 0 0\n";
		cout.flush();
		cin >> q1;
		cout << "Q 0 " << MAX << "\n";
		cout.flush();
		cin >> q2;
		ull temp = q1-q2;
		cout << "Q " << (MAX - temp)/2 << " 0\n";
		cout.flush();
		cin >> q3;
		cout << "Q " << (MAX - temp)/2 << " " << MAX << "\n";
		cout.flush();
		cin >> q4;
		xl = q1 - q3;
		xu = MAX - (q2 - q3);
		yl = q3;
		yu = MAX - q4;
		cout << "A " << xl << " " << xu << " " << yl << " " << yu << "\n";
		cout.flush();
	}
	return 0;
}

