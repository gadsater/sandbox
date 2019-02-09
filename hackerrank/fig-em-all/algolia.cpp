#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
#define MODN 1000000007
using namespace std;

void fast() {
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
}

int main() {
  fast();
  ll t;
  cin >> t;
  while (t--) {
    ll x;
    cin >> x;
    ll sum = 0;
    for (int i=1; i<=x; i++) {
      sum = (sum + (i * (x/i)) % MODN) % MODN;
    }
    cout << sum << endl;
  }
  return 0;
}

