#include "day07.h"
#include <map>
#include <set>
#include <sstream>
#include <utility>
#include <queue>

int Day07Solver::kDay() {
    return 7;
}

namespace {

    struct Direction {
        int dx;
        int dy;
        Direction(int dx_, int dy_) : dx(dx_), dy(dy_) {}
    };

    const Direction LEFT(-1, 0);
    const Direction RIGHT(1, 0);
    const Direction DOWN(0, 1);
    const Direction UP(0, -1);

    struct Position {
        int x;
        int y;
        Position(int x_, int y_) : x(x_), y(y_) {}

        Position move(Direction dir) {
            return Position(x + dir.dx, y + dir.dy);
        }

        bool operator<(const Position& other) const {
            return std::tie(x, y) < std::tie(other.x, other.y);
        }
    };

    struct Item {
        char value;
        Position position;
        Item(char value, Position position) : value(value), position(position) {}

        bool is_source() {
            return value == 'S';
        }

        bool is_splitter() {
            return value == '^';
        }
    };

    struct Grid {
        std::vector<std::vector<Item>> cells;
        Grid(std::vector<std::vector<Item>> cells) : cells(cells) {}

        bool has(Position pos) {
            return pos.y >= 0 && pos.y < static_cast<int>(cells.size()) &&
                   pos.x >= 0 && pos.x < static_cast<int>(cells[pos.y].size());
        }

        Item get(Position pos) {
            return cells[pos.y][pos.x];
        }

        void print(Position curr, std::set<Position> *visited = nullptr) {
            for (auto row : cells) {
                for (auto it : row) {
                    char v = it.value;

                    bool is_curr = (it.position.x == curr.x && it.position.y == curr.y);
                    if (is_curr) {
                        v = 'X';
                    }

                    bool is_visited = (visited && visited->find(it.position) != visited->end());
                    if (is_visited && !it.is_splitter()) {
                        v = '|';
                    }

                    std::cout << v;
                }
                std::cout << std::endl;
            }
        }
    };

    int count_splitters(Grid grid, Position start) {
        std::set<std::pair<Position, int>> visited; // (position, step_count)
        std::set<Position> splitter_positions; // Track unique splitter positions
        std::queue<std::pair<Position, int>> q; // (position, step_count)

        q.push({start, 0});

        while (!q.empty()) {
            auto [curr, steps] = q.front();
            q.pop();

            if (!grid.has(curr)) {
                continue;
            }

            // Check if already visited at this step
            if (!visited.insert({curr, steps}).second) {
                continue;
            }

            Item item = grid.get(curr);
            if (item.is_splitter()) {
                splitter_positions.insert(curr); // Track unique position
                auto l = curr.move(LEFT);
                auto r = curr.move(RIGHT);

                // Always add to queue (merging happens via visited check above)
                q.push({l, steps + 1});
                q.push({r, steps + 1});
            } else {
                // Always add to queue
                q.push({curr.move(DOWN), steps + 1});
            }
        }

        return splitter_positions.size();
    }

    long count_timelines(Grid grid, Position curr, std::map<Position, long>& memo) {
        if (memo.find(curr) != memo.end()) {
            return memo[curr];
        }

        if (!grid.has(curr)) {
            return 1;
        }

        Item item = grid.get(curr);
        if (item.is_splitter()) {
            auto l = curr.move(LEFT);
            auto r = curr.move(RIGHT);
            long res = count_timelines(grid, l, memo) + count_timelines(grid, r, memo);
            memo[curr] = res;
            return res;
        }

        long res = count_timelines(grid, curr.move(DOWN), memo);
        memo[curr] = res;
        return res;
    }

}

std::string Day07Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<std::vector<Item>> cells;
    Position beam(0,0);
    while (std::getline(ss, line)) {
        if (line.empty()) continue; // Skip empty lines
        std::vector<Item> row;
        for (char c : line) {
            Position postion(row.size(), cells.size());
            Item item(c, postion);
            row.push_back(item);
            if (item.is_source()) {
                beam = postion;
            }
        }
        cells.push_back(row);
    }

    Grid grid(cells);
    int splitters = count_splitters(grid, beam);
    return std::to_string(splitters);
}

std::string Day07Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<std::vector<Item>> cells;
    Position beam(0,0);
    while (std::getline(ss, line)) {
        if (line.empty()) continue; // Skip empty lines
        std::vector<Item> row;
        for (char c : line) {
            Position postion(row.size(), cells.size());
            Item item(c, postion);
            row.push_back(item);
            if (item.is_source()) {
                beam = postion;
            }
        }
        cells.push_back(row);
    }

    Grid grid(cells);
    std::map<Position, long> memo;
    long timelines = count_timelines(grid, beam, memo);
    return std::to_string(timelines);
}
