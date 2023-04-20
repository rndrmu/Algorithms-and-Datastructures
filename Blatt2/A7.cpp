// euclidian algorithm
#include <iostream>

/*
 * 1. Set m = a and n = b
 * 2. if m < n, switch m with n
 * 3. calculate r = m - n
 * 4. set m = n, n = r
 * 5. if r != 0 continue with step 2
 * */
int euclidian(int a, int b) {
    int m = a;
    int n = b;
    if (m < n) {
        int tmp = m;
        m = n;
        n = tmp;
    }
    int r = m - n;
    m = n;
    n = r;
    if (r != 0) {
        return euclidian(m, n);
    } else {
        return n;
    }
}

int main() 
{
    int a = 0;
    int b = 0;
    std::cout << "Enter a: ";
    std::cin >> a;
    std::cout << "Enter b: ";
    std::cin >> b;
    std::cout << "ggT: " << euclidian(a, b) << std::endl;
}
