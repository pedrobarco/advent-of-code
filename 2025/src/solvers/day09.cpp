#include "day09.h"
#include <cstdio>
#include <sstream>
#include <vector>

int Day09Solver::kDay() {
    return 9;
}

struct Position {
    int x;
    int y;
    Position(int x_, int y_) : x(x_), y(y_) {}
};

struct Rectangle {
    Position p1;
    Position p2;

    long area() const {
        long width = std::abs(p1.x - p2.x) + 1;
        long height = std::abs(p1.y - p2.y) + 1;
        return width * height;
    }
};

std::string Day09Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<Position> points;
    while (std::getline(ss, line)) {
        int x, y;
        std::sscanf(line.c_str(), "%d,%d", &x, &y);
        points.push_back(Position(x, y));
    }

    long max_area = 0;
    for (size_t i = 0; i < points.size(); i++) {
        for (size_t j = i + 1; j < points.size(); j++) {
            Position p1 = points[i];
            Position p2 = points[j];
            Rectangle rect{p1, p2};
            long area = rect.area();
            if (area > max_area) {
                max_area = area;
            }
        }
    }

    return std::to_string(max_area);
}

std::string Day09Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<Position> points;
    while (std::getline(ss, line)) {
        int x, y;
        std::sscanf(line.c_str(), "%d,%d", &x, &y);
        points.push_back(Position(x, y));
    }

    long max_area = 0;
    for (size_t i = 0; i < points.size(); i++) {
        for (size_t j = i + 1; j < points.size(); j++) {
            Position p1 = points[i];
            Position p2 = points[j];
            Rectangle rect{p1, p2};
            // TODO: skip if rectangle is out of bounds
            long area = rect.area();
            if (area > max_area) {
                max_area = area;
            }
        }
    }

    return std::to_string(max_area);
}
