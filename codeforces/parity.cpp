#include <bits/stdc++.h>
#define ll long long
using namespace std;

int main() {
  ll b, k;
  bool bflag = 0, pflag = 0;
  cin >> b >> k;
  ll c;
  if (b % 2 == 1) 
    bflag = 1;
  
  if (bflag == 0) {
    for(int i=0; i<k; i++) {
      cin >> c;
    }
    if (c % 2 == 1) {
      pflag = 1;
    }
  } else {
    for(int i=0; i<k; i++) {
      cin >> c;
      if ((c % 2) == 1) {
        pflag = pflag ? 0 : 1;
      }
    }
  }
  if (pflag) {
    cout << "odd\n";
  } else {
    cout << "even\n";
  }
  return 0;
}

