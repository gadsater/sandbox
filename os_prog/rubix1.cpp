#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
#define fr(D,V,B,C,E,S) for(D V=B;V C E, V += S)
#define fri(D,V,B,E) for(D,V,B,"<",E,1)
using namespace std;

void backtrack(int jobs[], int n, int proc[], int p, int dp[], int i, double *ans) {
	double temp = 0;
	for (int j=0; j<p; j++) {
		temp = max(temp, (dp[j]*1.0)/proc[j]);
	}

	if (temp > *ans) {
		return;
	} else if (i == n) {
		if ( temp < *ans) {
			*ans = temp;
		}
		return;
	}
	else {
		for (int j = 0; j<p; j++) {
			dp[j] += jobs[i];
			backtrack(jobs, n, proc, p, dp, (i+1), ans);
			dp[j] -= jobs[i];
		}
	}
}

int main() {
	int t, n, p;
	cin >> t;
	while (t--) {
		double ans = 1e8;
		cin >> n;
		int jobs[n];
		for (int i=0; i<n; i++) {
			cin >> jobs[i];
		}
		cin >> p;
		int proc[p];
		for (int i=0; i<p; i++) {
			cin >> proc[i];
		}
		int dp[p+1] = {0};
		backtrack(jobs, n, proc, p, dp, 0, &ans);
		//printf("%6lf\n", ans);
		cout << ans << endl;
	}
	return 0;
}

