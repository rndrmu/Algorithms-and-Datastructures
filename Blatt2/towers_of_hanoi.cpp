 #include <iostream>
using namespace std;

void move_disc(int disc, char source, char dest) {
    cout << "Moving disc " << disc << " from tower " << source << " to tower " << dest << endl;
}

void towers_of_hanoi(int num_discs, char source, char temp, char dest) {
    if (num_discs == 1) {
        move_disc(1, source, dest);
    } else {
        towers_of_hanoi(num_discs - 1, source, dest, temp);
        move_disc(num_discs, source, dest);
        towers_of_hanoi(num_discs - 1, temp, source, dest);
    }
}

int main() {
    int num_discs;
    cout << "Enter number of discs: ";
    cin >> num_discs;

    towers_of_hanoi(num_discs, 'A', 'B', 'C');

    return 0;
}

