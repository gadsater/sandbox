#include <bits/stdc++.h>
#include <stdlib.h>
using namespace std;
#define ll long long

int main() {
	ll t, arr[26];
	cin >> t;
	while (t--) {
		memset(arr, 0, 26 * sizeof(long long));
		string str;
		cin >> str;
		for (long long i=0; i<str.length(); i++){
			arr[str[i] - 65]++;
		}
		sort(arr, arr + 26);
		reverse(arr , arr + 26);
		long long uniq_char = 0;
		for (int i=0; i<26; i++) {
			if (arr[i] != 0) {
				uniq_char++;
			} else {
				break;
			}
		}
		long long arr_sum = 0;
		for (int i=0; i<26; i++) {
			arr_sum += arr[i];
		}
		long long min = LLONG_MAX;
		for (int i=1; i<=uniq_char; i++) {
			long long sum = 0;
			if (arr_sum % i == 0) {
				long long avg = arr_sum / i;
				for(int j=0; j<i; j++) {
					sum += (avg - arr[j]);
				}
			}
			if (sum < min) {
				min = sum;
			}
		}
		cout << min << endl;	
	}
	return 0;
}
