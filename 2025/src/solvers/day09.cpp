#include "day09.h"
#include <cstdio>
#include <sstream>
#include <utility>
#include <vector>

int Day09Solver::kDay() {
    return 9;
}

namespace {

    struct Position {
        int x;
        int y;
        Position(int x_, int y_) : x(x_), y(y_) {}
        Position(const Position& other) = default;
        Position(Position&& other) = default;
        Position& operator=(const Position& other) = default;
        Position& operator=(Position&& other) = default;
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

    using Edge = std::pair<Position, Position>;
    using Shape = std::vector<Edge>;

    bool is_in_shape(const Position& p, const Shape& shape) {
        bool inside = false;
        for (const auto& edge : shape) {
            const Position& p1 = edge.first;
            const Position& p2 = edge.second;
            if ((p.x == p1.x && p1.x == p2.x && p.y >= std::min(p1.y, p2.y) && p.y <= std::max(p1.y, p2.y)) ||
                (p.y == p1.y && p1.y == p2.y && p.x >= std::min(p1.x, p2.x) && p.x <= std::max(p1.x, p2.x))) {
                return true;
            }
            if ((p1.y > p.y) != (p2.y > p.y) &&
                (p.x < (p2.x - p1.x) * (p.y - p1.y) / (p2.y - p1.y) + p1.x)) {
                inside = !inside;
            }
        }
        return inside;
    }

}

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

    Shape shape;
    for (size_t i = 0; i < points.size(); i++) {
        Edge edge = std::make_pair(points[i], points[(i + 1) % points.size()]);
        shape.push_back(edge);
    }

    long max_area = 0;
    for (size_t i = 0; i < points.size(); i++) {
        for (size_t j = i + 1; j < points.size(); j++) {
            Position p1 = points[i];
            Position p2 = points[j];
            Position p3 = {p1.x, p2.y};
            Position p4 = {p2.x, p1.y};

            bool in_shape = (
                is_in_shape(p1, shape) &&
                is_in_shape(p4, shape) &&
                is_in_shape(p2, shape) &&
                is_in_shape(p3, shape)
            );

            if (!in_shape) {
                continue;
            }

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
