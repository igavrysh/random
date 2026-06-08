#include <iostream>
#include <vector>
#include <sstream>
#include <string>
#include <algorithm>

using namespace std;
int main(int argc, char const *argv[]) {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    int n;
    cin >> n;
    vector<vector<int>>data{};
    for (int i=0;i<n;i++) {
        int curr_size = 0;
        cin >> curr_size;
        vector<int> curr{};
        for (int j=0;j<curr_size;j++) {
            int num;
            cin >> num;
            curr.push_back(num);
        }
        data.push_back(curr);
    }
    for (int i=0;i<n;i++) {
        vector<int>& s = data[i];
        sort(data[i].begin(), data[i].end(), greater<int>());
        // cout << "sorted data " << i << ":";
        // for (int j = 0; j < data[i].size(); j++) {
        //     cout << data[i][j] << " ";
        // } 
        // cout << endl;
        bool valid = true;
        for (int j=0;j<data[i].size()-2;j++) {

            if (data[i][j] % data[i][j+1] != data[i][j+2]) {
                valid = false;
                break;
            }
        }
        if (valid) {
            cout << data[i][0] << " " << data[i][1] << endl;
        } else {
            cout << -1 << endl;
        }
    }
}
