#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
using namespace std;

ull ocpl(ull n) 
{ 
   int number_of_bits = floor(log2(n))+1; 
   return ((1 << number_of_bits) - 1) ^ n; 
} 
  
int main() {
  ull q, a;
  cin >> q;
  while (q--) {
    int flag = 0;
    cin >> a;
    for (int i=1; i<26; i++) {
      ull b = (2 << i) - 1;
      if (a == b) {
        flag = 1;
        break;
      }
    }
    if (flag) {
      int pflag = 1;
      for (int i=2; i<sqrt(a); i++) {
        if (a % i == 0) {
          pflag = 0;
          a = a / i;
          break;
        }
      }
      cout << (pflag ? 1 : a) << endl; 
    } else {
      cout << (a ^ ocpl(a)) << endl;
    }
  }
  return 0;
}

