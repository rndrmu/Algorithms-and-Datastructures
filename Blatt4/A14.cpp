class Vektor 
{ 
private: 
    int dimension; 
    int *daten; 
public: 
    Vektor(int dim); 
    virtual ~Vektor(); 
    void set(int i, int val); 
    int get(int i); 
    int bin_suche(int sw); 
};

Vektor::Vektor(int dim) : dimension(dim) {
    daten = new int[dim]; 
    for (int i = 0; i < dimension; ++i) {
        daten[i] = 0;
    }
}

Vektor::~Vektor() {
    delete[] daten; 
}

void Vektor::set(int i, int val) {
    if (i >= 0 && i < dimension) {
        daten[i] = val;
    }
}

int Vektor::get(int i) {
    if (i >= 0 && i < dimension) {
        return daten[i];
    }
    return -1; 
}

int Vektor::bin_suche(int sw) { 
    int l_u = 0, l_o = dimension - 1; 
    
    while (l_u <= l_o) {
        int mid = l_u + (l_o - l_u) / 2; 
        
        if (daten[mid] == sw) {
            return mid; 
        } else if (daten[mid] < sw) {
            l_u = mid + 1; 
        } else {
            l_o = mid - 1; 
        }
    }
    
    return -1; 
}
