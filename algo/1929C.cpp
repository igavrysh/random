/*
2 1 7
coins: 7
put 1: coins: 6 lost
put 1: coins: 5 win: +2 coins: 7
put 1: coins: 6 lost
put 1: coins: 5 win: +2 coints: 7
no winning

coins: 7
put 1: coins: 6 lost
put: 6 coins: 0 win +12
put 1: coins

coins: 7 lets put 3 every time

put: 3 coins:4 lost

(initial_coins - lost)*a > initial_coins ?

3 3 6

6
put 1 coin coins: 5 lost
put 1 coin coins: 4 lost
put 1 coin coins: 3 lost
put 3 coin coins: 0 win +3*3 = 9

put 1 coin coins: 8 lost
put 1 coin coins: 7 lost
put 1 coin coins: 6 lost
put 6 coins coins: 0 win +6*3 = 18
*/
#include <iostream>
#include <cmath>
using namespace std;
bool* sol(int n, int **queries) {
    bool *output = new bool[n];
    for (int q = 0; q < n; q++) {
        int a = queries[q][0];
        int louse_series = queries[q][1];
        int initial_coins = queries[q][2];
        int current_capital = initial_coins;
        bool res = true;
        for (int i = 0; i < louse_series; i++) {
            int to_bet = (initial_coins + 1 - current_capital) / (a-1);
            if ((initial_coins + 1 - current_capital) % (a-1) != 0 || to_bet == 0) {
                to_bet++;
            }
            if (to_bet > current_capital) {
                res = false;
                break;
            }
            //cout << "to_bet:" << to_bet <<  ";curret_c:" 
            // << current_capital  << ";if_win:" << (current_capital - to_bet + to_bet*a) << "\n";

            current_capital -= to_bet;
        }
        if (((long long)current_capital * (long long)a) <= ((long long)initial_coins)) {
            res = false;
        }
        output[q] = res;
    }
    return output;
}

int main(int argc, char const *argv[]) {
    /*
    int n = 1;
    int input[1][3] = {{21, 91, 1000000000}};
    int rows = 1, cols = 3;
    int** input_dyn_a = new int*[rows];
    for (int i = 0; i < rows; ++i) {
        input_dyn_a[i] = new int[cols];
        for (int j = 0; j < cols; ++j) {
            input_dyn_a[i][j] = input[i][j];
        }
    }
    bool* output = sol(n, input_dyn_a);
    int t = 1;
    */
    int n;
    cin >> n;
    int** input = new int*[n];
    for (int i = 0; i < n; i++) {
        input[i] = new int[3];
        cin >> input[i][0];
        cin >> input[i][1];
        cin >> input[i][2];
    }
    bool* output = sol(n, input);
    for (int i = 0; i < n; i++) {
        if (output[i] == true) {
            cout << "YES\n";
        } else {
            cout << "NO\n";
        }
    }
    return 0;
}