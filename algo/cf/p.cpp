#include <iostream>
#include <vector>
using namespace std;
const int MOD = 1000000007;
long long combs(int ptr, int k, vector<vector<int>> a, vector<vector<int>> dp) {
    if (dp[ptr][k] != -2) {
        return dp[ptr][k];
    }
    if (ptr>=a.size() || k<=0) {
        if (ptr==a.size() && k==0) {
            dp[ptr][k] = 1;
            return 1;
        }
        dp[ptr][k] = -1;
        return -1;
    }
    long long acc = 0;
    cout << "a[i]: " << a[ptr][0] << " " << a[ptr][1] << endl;
    for (int i=a[ptr][0];i<=a[ptr][1];i++) {
        long long res = combs(ptr+1, k-i, a, dp);
        if (res!=-1) {
            acc = (acc + res)%MOD;
        }
    }
    dp[ptr][k] = acc;
    return acc;
}
signed main() {
    int n,k;
    cin >> n >> k;
    vector<vector<int>> a(n, vector<int>(2));
    for (int i=0;i<n;i++) {
        cin >> a[i][0] >> a[i][1];
    }
    vector<vector<int>> dp(n+1, vector<int>(k+1, -2));
    cout << combs(0,k,a, dp);
}