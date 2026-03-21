#include <iostream>
#include <cmath>
using namespace std;
int main(int argc, char const *argv[]) {
    int segments[] = {6, 2, 5, 5, 4, 5, 6, 3, 7, 6};
    int a, b;
    cin >> a >> b;
    long output = 0;
    for (int i=a; i<=b; i++) {
        int num = i;
        while (num > 0) {
            output += segments[num%10];
            num = num/10;
        }
    }
    cout << output;
}