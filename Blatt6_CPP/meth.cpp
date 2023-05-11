#include "core.cpp"
#include <iostream>
#include <string>
#include <sstream>
#include <cctype>
#include <map>

std::string infix_to_postfix(const std::string& infix)
{
    std::istringstream iss(infix);
    std::ostringstream oss;
    Stack<char> stack;
    std::map<char, int> precedence{{'+', 1}, {'-', 1}, {'*', 2}, {'/', 2}, {'^', 3}};

    auto is_operator = [](char c) -> bool {
        return c == '+' || c == '-' || c == '*' || c == '/' || c == '^';
    };

    auto higher_precedence = [&](char op1, char op2) -> bool {
        return precedence[op1] >= precedence[op2];
    };

    char c;
    while (iss >> c) {
        if (std::isdigit(c) || std::isalpha(c)) {
            oss << c;
        } else if (c == '(') {
            stack.push(c);
        } else if (c == ')') {
            while (!stack.empty() && stack.top() != '(') {
                oss << stack.pop();
            }

            if (!stack.empty() && stack.top() == '(') {
                stack.pop();
            } else {
                throw std::invalid_argument("Mismatched parentheses");
            }
        } else if (is_operator(c)) {
            while (!stack.empty() && stack.top() != '(' && higher_precedence(stack.top(), c)) {
                oss << stack.pop();
            }

            stack.push(c);
        } else {
            throw std::invalid_argument("Invalid character in input");
        }
    }

    while (!stack.empty()) {
        if (stack.top() == '(') {
            throw std::invalid_argument("Mismatched parentheses");
        }

        oss << stack.pop();
    }

    return oss.str();
}

double evaluate_postfix(const std::string& postfix)
{
    std::istringstream iss(postfix);
    Stack<double> stack;

    char c;
    while (iss >> c) {
        if (std::isdigit(c)) {
            double num = 0;
            while (std::isdigit(c)) {
                num = num * 10 + (c - '0');
                iss >> c;
            }
            iss.putback(c);
            stack.push(num);
        } else if (c == '+' || c == '-' || c == '*' || c == '/' || c == '^') {
            double op2 = stack.pop();
            double op1 = stack.pop();
            double result;

            switch (c) {
                case '+':
                    result = op1 + op2;
                    break;
                case '-':
                    result = op1 - op2;
                    break;
                case '*':
                    result = op1 * op2;
                    break;
                case '/':
                    if (op2 == 0) {
                        throw std::invalid_argument("Division by zero");
                    }
                    result = op1 / op2;
                    break;
                case '^':
                    result = std::pow(op1, op2);
                    break;
            }

            stack.push(result);
        }
    }

    if (stack.length() != 1) {
        throw std::invalid_argument("Invalid postfix expression");
    }

    return stack.pop();
}



int main()
{
    std::string infix = "a+b*c-(d/e+f)*g";
    std::string postfix = infix_to_postfix(infix);
    std::cout << "Infix expression: " << infix << std::endl;
    std::cout << "Postfix expression: " << postfix << std::endl;
    return 0;
}