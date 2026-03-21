// https://codeforces.com/problemset/problem/357/A

#include <iostream>
using namespace std;
int main(int argc, char const *argv[]) {
    int n;
    cin >> n;
    int* c = new int[n];
    for (int i=0;i<n;i++) {
        cin>>c[i];
    }
    int x,y;
    cin>>x>>y;
    int acc_x = 0, acc_y = 0;
    bool fill_first = true;
    int res = 0;
    int interim_res = 0;
    int total_sum = 0;
    for (int i=0;i<n;i++) {
        total_sum += c[i];
    }
    for (int i=0;i<n;i++) {
        if (fill_first) {
            if (acc_x+c[i]<=y && total_sum-(acc_x+c[i])>=x) {
                acc_x += c[i];
            } else {
                if (acc_x < x) {
                    break;
                }
                fill_first = false;
                interim_res = i+1;
            }
        }
        if (!fill_first) {
            if (acc_y+c[i]<=y) {
                acc_y += c[i];
            } else {
                break;
            }
        }
        if (i==n-1) {
            res = interim_res;
        }
    }
    cout<<res;
}


