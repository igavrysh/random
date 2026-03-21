#include <iostream>
#include <cmath>
using namespace std;
int main(int argc, char const *argv[]) {
    long long table[100000][2];
    int j = 0;
    for (int i = 1; i < 100000; i++) {
        long long i_sq = i;
        i_sq = i_sq * i_sq;
        if (i_sq%2==1) {
            long long a = i;
            long long b = (i_sq + 1) / 2;
            long long c = i_sq - b;
            if (a > 0 && b > 0 && c > 0) {
                table[j][0] = max(c, max(a, b));
                table[j][1] = 1 + (j-1 >= 0 ? table[j-1][1] : 0);
                j++;
            }
        }
    }
    /*
    cout << "max_idx" << j << "\n";
    for (int i = 0; i <= j; i++) {
        cout << table[i][0] << ":" << table[i][1] << "\n";
    }
    */

    int max_idx = j-1;
    int q;
    cin >> q;

    for (int i = 0; i < q; i++) {
        long long n;
        cin >> n;
        int good = -1;
        int bad = max_idx+1;
        while (bad - good > 1) {
            long m = good + (bad - good)/2;
            if (table[m][0] == n) {
                good = m;
                break;
            }
            if (table[m][0] > n) {
                bad = m;
            } else {
                good = m;
            }
        }
        if (good == -1) {
            cout << 0 << "\n";
        } else {
            cout << table[good][1] << "\n";
        }
    }


    std::cout << "Size of int: " << sizeof(int) << " bytes\n";
    std::cout << "Size of long: " << sizeof(long) << " bytes\n";
    std::cout << "Size of long long: " << sizeof(long long) << " bytes\n";

}