#include <iostream>
#include <map>
#include <limits>
using namespace std;
int main(int argc, char const *argv[]) {
    ios::sync_with_stdio(0);
    cin.tie(0);
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        int m, k;
        cin >> m;
        cin >> k;
        bool found = false;
        map<int, int> map;
        int number;
        for (int j = 0; j < m; j++) {
            int a;
            cin >> a;
            if (auto search = map.find(a); search != map.end()) {
                found = true;
                cin.ignore(numeric_limits<streamsize>::max(), '\n');
                break;
            } else {
                map.insert(std::pair<int, int> (k+a, 1));
                map.insert(std::pair<int, int> (a-k, 1));
            }
        }
        cout << (found ? "YES" : "NO") << "\n";
    }
    return 0;
}
