#include "day09.h"
#include <cstdio>
#include <map>
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

    bool is_edge_in_shape(const Edge& edge, const Shape& shape) {
        const Position& edge_p1 = edge.first;
        const Position& edge_p2 = edge.second;

        // Always check both endpoints
        if (!is_in_shape(edge_p1, shape) || !is_in_shape(edge_p2, shape)) {
            return false;
        }

        // Handle horizontal edge (same y coordinate)
        if (edge_p1.y == edge_p2.y) {
            int min_x = std::min(edge_p1.x, edge_p2.x);
            int max_x = std::max(edge_p1.x, edge_p2.x);

            // Sample every 1000 units
            for (int x = min_x + 1000; x < max_x; x += 1000) {
                Position p(x, edge_p1.y);
                if (!is_in_shape(p, shape)) {
                    return false;
                }
            }
            return true;
        }

        // Handle vertical edge (same x coordinate)
        if (edge_p1.x == edge_p2.x) {
            int min_y = std::min(edge_p1.y, edge_p2.y);
            int max_y = std::max(edge_p1.y, edge_p2.y);

            // Sample every 1000 units
            for (int y = min_y + 1000; y < max_y; y += 1000) {
                Position p(edge_p1.x, y);
                if (!is_in_shape(p, shape)) {
                    return false;
                }
            }
            return true;
        }

        // Diagonal edges shouldn't occur for axis-aligned rectangles
        return false;
    }

    struct EdgeKey {
        Position p1, p2;

        // Constructor automatically normalizes the edge
        EdgeKey(const Edge& edge)
            : p1((edge.first.x < edge.second.x || (edge.first.x == edge.second.x && edge.first.y < edge.second.y)) ? edge.first : edge.second),
              p2((edge.first.x < edge.second.x || (edge.first.x == edge.second.x && edge.first.y < edge.second.y)) ? edge.second : edge.first) {
            // p1 and p2 are now initialized with normalized positions
        }

        // Comparison operator for std::map
        bool operator<(const EdgeKey& other) const {
            if (p1.x != other.p1.x) return p1.x < other.p1.x;
            if (p1.y != other.p1.y) return p1.y < other.p1.y;
            if (p2.x != other.p2.x) return p2.x < other.p2.x;
            return p2.y < other.p2.y;
        }
    };

    bool is_edge_in_shape_cached(const Edge& edge,
                                   const Shape& shape,
                                   std::map<EdgeKey, bool>& cache) {
        EdgeKey key(edge);

        // Check cache first
        auto it = cache.find(key);
        if (it != cache.end()) {
            return it->second;  // Cache hit!
        }

        // Cache miss - compute and store
        bool result = is_edge_in_shape(edge, shape);
        cache[key] = result;
        return result;
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

    std::map<EdgeKey, bool> cache;
    long max_area = 0;
    for (size_t i = 0; i < points.size(); i++) {
        for (size_t j = i + 1; j < points.size(); j++) {
            Position p1 = points[i];
            Position p2 = points[j];
            Position p3 = {p1.x, p2.y};
            Position p4 = {p2.x, p1.y};

            // Strategy A: Early exit if this rectangle can't beat current max area
            Rectangle rect{p1, p2};
            long area = rect.area();
            if (area <= max_area) {
                continue;
            }

            // Strategy B: Check all 4 edges of the rectangle (with caching)
            bool all_edges_valid = (
                is_edge_in_shape_cached({p1, p3}, shape, cache) &&
                is_edge_in_shape_cached({p3, p2}, shape, cache) &&
                is_edge_in_shape_cached({p2, p4}, shape, cache) &&
                is_edge_in_shape_cached({p4, p1}, shape, cache)
            );

            if (!all_edges_valid) {
                continue;
            }

            max_area = area;
        }
    }


    return std::to_string(max_area);
}
