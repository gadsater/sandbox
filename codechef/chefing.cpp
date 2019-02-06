#include <bits/stdc++.h>
using namespace std;

int main() {
	int t;
	cin >> t;
	while (t--) {
		int n;
		cin >> n;
		long long *arr = (long long *) calloc(26, sizeof(long long));
		for (int i = 0; i < n; i++) {
			char str[200];
			cin >> str;
			int j = 0;
			int flarr[26] = {0};
			while (*(str + j)) {
				if(flarr[*(str + j) - 96] == 0) {
					arr[*(str + j) - 96]++;
					flarr[*(str + j) - 96] = 1;
				}
				j++;
			}
		}
		int count = 0;
		for (int i = 0; i < 26; i++) {
			if (arr[i] == n) {
				count++;
			}
		}
		cout << count << "\n";
	}
	return 0;
}

