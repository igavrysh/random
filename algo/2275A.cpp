#include <iostream>
#include <cmath>
using namespace std;
int**  sol(int n, int **queries) {
    int** output = new int*[n];
    for (int i = 0; i < n; i++) {
        output[i] = new int[2];
        output[i][0] = queries[i][0];
        output[i][1] = queries[i][1] + queries[i][2];
    }
    return output;
}
int main(int argc, char const *argv[]) {
    int n;
    cin >> n;
    int** input = new int*[n];
    for (int i = 0; i < n; i++) {
        input[i] = new int[3];
        cin >> input[i][0];
        cin >> input[i][1];
        cin >> input[i][2];
    }
    int**  output = sol(n, input);
    for (int i = 0; i < n; i++) {
        cout << output[i][0] << " " << output[i][1] << "\n";
    }
    return 0;
}