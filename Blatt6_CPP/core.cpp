#include <vector>

template <class item_type>
class Stack
{
private:
    std::vector<item_type> data;
    int anz_items;

public:
    Stack() : anz_items(0) {}
    ~Stack() {}

    void push(item_type& r)
    {
        data.push_back(r);
        anz_items++;
    }

    item_type pop()
    {
        if (anz_items == 0) {
            throw std::out_of_range("Stack is empty");
        }

        item_type result = data.back();
        data.pop_back();
        anz_items--;
        return result;
    }

    item_type top()
    {
        if (anz_items == 0) {
            throw std::out_of_range("Stack is empty");
        }

        return data.back();
    }

    int length()
    {
        return anz_items;
    }

    bool empty()
    {
        return anz_items == 0;
    }
};
