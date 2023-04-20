#include <iostream>
#include <cmath>
#include <iomanip>

double arctan_recursive(double x, int n) {
    if (n == 0) {
        return x;
    } else {
        double term = pow(-1, n) * pow(x, 2*n+1) / (2*n+1);
        return term + arctan_recursive(x, n-1);
    }
}

double _pi() {
    double arctan_one = arctan_recursive(1, 1000);
    return 2 * arctan_one;
}



int main() {
    double pi = 2 * _pi(); // umadbro?
    std::cout << "The first digits of pi = " << std::fixed << std::setprecision(15) << pi << std::endl;
    return 0;
}
