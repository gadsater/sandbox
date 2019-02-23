#include <bits/stdc++.h>
#define ll long long
#define ull unsigned long long
#define fr(D,V,B,C,E,S) for(D V=B;V C E, V += S)
#define fri(D,V,B,E) for(D,V,B,"<",E,1)
using namespace std;

int main() {
	unsigned int r, c, l, h;
	cin >> r >> c >> l >> h;
	int pizza[r][c], count_m = 0, count_t = 0;
	char temp;
	for (int i = 0; i < r; i++) {
		for (int j = 0; j < c; j++) {
			cin >> temp;
			pizza[i][j] = temp == 'T' ? 1 : 0; // 1 -> tomato, 0 -> mushroom
			temp == 'T' ? count_t++; count_m++;
		}
	}


	return 0;
}

