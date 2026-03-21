#include <iostream>
#include <cmath>
using namespace std;
int main(int argc, char const *argv[]) {
    int n;
    int num;
    cin >> n;
    for (int i=0;i<n;i++) {
        cin >> num;
        int curr = 1;
        if (num == 1 || pow(3, (num-1)) > pow(10, 9)) {
            cout << "NO\n";
            continue;
        } else {
            cout << "YES\n";
        }
        for (int i = 0; i<num; i++) {
            cout << curr;
            if (i != num-1) {
                cout << " ";
            } else {
                cout << "\n";
            }
            curr *= 3;
        }
    }
    return 0;
}
