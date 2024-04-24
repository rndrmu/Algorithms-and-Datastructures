// T(n) = n ∗ T(n − 1) + n

// iterative version
void calc_t(int n) {
    int t = 1;
    for (int i = 1; i <= n; ++i) {
        t = i * t + i;
    }
    std::cout << t << std::endl;
}


// recursive version
int calc_t_rec(int n) {
    if (n == 1) {
        return 1;
    }
    return n * calc_t_rec(n - 1) + n;
}