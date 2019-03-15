#include <bits/stdc++.h> 
using namespace std;

const int N = 100000;  
  
long long n; // array size 
  
// Max size of tree 
long long tree[2 * N]; 
  
// function to build the tree 
void build( long long arr[])  
{  
    // insert leaf nodes in tree 
    for (long long i=0; i<n; i++)     
        tree[n+i] = arr[i]; 
      
    // build the tree by calculating parents 
    for (long long i = n - 1; i > 0; --i)      
        tree[i] = tree[i<<1] + tree[i<<1 | 1];     
} 
  
// function to update a tree node 
void updateTreeNode(long long p, long long value)  
{  
    // set value at position p 
    tree[p+n] = value; 
    p = p+n; 
      
    // move upward and update parents 
    for (long long i=p; i > 1; i >>= 1) 
        tree[i>>1] = tree[i] + tree[i^1]; 
} 
  
// function to get sum on interval [l, r) 
int query(long long l, long long r)  
{  
    long long res = 0; 
      
    // loop to find the sum in the range 
    for (l += n, r += n; l < r; l >>= 1, r >>= 1) 
    { 
        if (l&1)  
            res += tree[l++]; 
      
        if (r&1)  
            res += tree[--r]; 
    } 
      
    return res; 
} 
  
// driver program to test the above function  
int main()  
{ 
		cin >> n;
		long long a[n];
		for (long long i = 0; i<n; i++)
			cin >> a[i];

		long long k[n-1];
		for (long long i = 0; i<(n-1); i++)
			cin >> k[i];

    build(a);
    long long q;
		cin >> q;
		while (q--) {
			char ch;
			long long i, x;
			cin >> ch >> i >> x;
			if (ch == '+') {
				long long ind = i-1;
				updateTreeNode(ind, tree[n+ind] + x);
				for (long long j = ind; j < (n-1); j++) {
					if (tree[n+j+1] < (tree[n+j] + k[j])) {
						updateTreeNode(j+1, (tree[n+j] + k[j]));
					}
				}
			}
			else 
				cout << query((i-1), x) << "\n"; 
		}
    return 0; 
} 

