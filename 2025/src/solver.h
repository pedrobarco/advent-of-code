#pragma once

#include <string>
#include <iostream>

class Solver {
    public:
        virtual ~Solver() = default;
        virtual int kDay() = 0;
        virtual std::string one(std::string input) = 0;
        virtual std::string two(std::string input) = 0;
        void solve(std::string input_one, std::string input_two);
};


