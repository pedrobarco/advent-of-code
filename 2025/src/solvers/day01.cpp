#include "day01.h"
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>

int Day01Solver::kDay() {
    return 1;
}

int Day01Solver::mod(int x, int m) {
    int r = x%m;
    return r<0 ? r+m : r;
}

int Day01Solver::floor_div(int a, int b) {
    int q = a / b;
    int r = a % b;
    if (r != 0 && ((r > 0) != (b > 0))) {
        q--;
    }
    return q;
}

std::string Day01Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    int curr = 50;
    int pass = 0;
    while (std::getline(ss, line, '\n')) {
        char letter = line[0];
        int number = std::stoi(line.substr(1));

        if (letter == 'R') {
            curr += number;
        } else if (letter == 'L') {
            curr -= number;
        } else {
            throw std::runtime_error("Invalid letter");
        }

        int zero = this->mod(curr, 100) == 0;
        if (zero) {
            pass++;
        }
    }

    return std::to_string(pass);
}

std::string Day01Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    int prev = 50;
    int pass = 0;
    while (std::getline(ss, line, '\n')) {
        char letter = line[0];
        int number = std::stoi(line.substr(1));

        int from = this->floor_div(prev, 100);

        int curr {};
        if (letter == 'R') {
            curr = prev + number;
        } else if (letter == 'L') {
            curr = prev - number;
        } else {
            throw std::runtime_error("Invalid letter");
        }

        int to = this->floor_div(curr, 100);
        int d = std::abs(to - from);
        bool is_zero = this->mod(curr, 100) == 0;
        bool was_zero = this->mod(prev, 100) == 0;
        bool left_zero = curr < prev && letter == 'L';
        if(is_zero && left_zero){
            d++;
        }
        if(was_zero && left_zero){
            d--;
        }

        pass += d;
        prev = curr;
    }

    return std::to_string(pass);
}

