#include <bits/stdc++.h>
using namespace std;

string removePR(string s, int v, int& res) {
    stack<char> st;
    
    for (char c : s) {
        if (c == 'r') {
            if ((not st.empty()) and st.top() == 'p') {
                res += v;
                st.pop();
            } else {
                st.push(c);
            }
        } else {
            st.push(c);
        }
    }
    
    string out;
    while (not st.empty()) {
        out.push_back(st.top());
        st.pop();
    }
    
    reverse(out.begin(), out.end());
    
    return out;
}

string removeRP(string s, int v, int& res) {
    stack<char> st;
    
    for (char c : s) {
        if (c == 'p') {
            if ((not st.empty()) and st.top() == 'r') {
                res += v;
                st.pop();
            } else {
                st.push(c);
            }
        } else {
            st.push(c);
        }
    }
    
    string out;
    while (not st.empty()) {
        out.push_back(st.top());
        st.pop();
    }
    
    reverse(out.begin(), out.end());
    
    return out;
}

int main() {
    int t;
    cin >> t;
    while (t--) {
        string s;
        cin >> s;
        
        int x, y; // x -> pr, y -> rp
        cin >> x >> y;
        
        int res = 0;
        
        if (x > y) {
            s = removePR(s, x, res);
            s = removeRP(s, y, res);
        } else {
            s = removeRP(s, y, res);
            s = removePR(s, x, res);
        }
        
        cout << res << "\n";
    }
    
	return 0;
}
