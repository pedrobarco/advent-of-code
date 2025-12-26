#include "../solver.h"

class Day01Solver : public Solver {
    public:
        int kDay() override;
        std::string one(std::string input) override;
        std::string two(std::string input) override;
    private:
        int mod(int x, int m);
        int floor_div(int a, int b);
};


