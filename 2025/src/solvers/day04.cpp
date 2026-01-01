#include "day04.h"
#include <sstream>
#include <string>
#include <vector>

int Day04Solver::kDay() {
    return 4;
}

namespace {

    class Direction {
        public:
            int dx;
            int dy;
            Direction(int dx, int dy) : dx(dx), dy(dy) {}
    };

    const std::vector<Direction> directions = {
        Direction(0, -1),  // up
        Direction(0, 1),   // down
        Direction(-1, 0),  // left
        Direction(1, 0),   // right
        Direction(-1, -1), // up-left
        Direction(1, -1),  // up-right
        Direction(-1, 1),  // down-left
        Direction(1, 1)    // down-right
    };

    class Position {
        public:
            int x;
            int y;
            Position(int x, int y) : x(x), y(y) {}
    };

    class Item {
        public:
            char value;
            Position position;
            bool is_paper() {
                return value == '@';
            }
            Item(char value, Position position) : value(value), position(position) {}
    };

    class Grid {
        private:
            std::vector<std::vector<Item>> cells;
        public:
            Item get(Position pos) {
                return cells[pos.y][pos.x];
            }
            void remove(Position pos) {
                cells[pos.y][pos.x] = Item('.', pos);
            };
            std::vector<Item> neighbours(Item item) {
                std::vector<Item> result;

                for (auto& dir : directions) {
                    int nx = item.position.x + dir.dx;
                    int ny = item.position.y + dir.dy;
                    if (nx >= 0 && ny >= 0 && ny < (int)cells.size() && nx < (int)cells[ny].size()) {
                        result.push_back(cells[ny][nx]);
                    }
                }

                return result;
            }
            Grid(std::vector<std::vector<Item>> cells) : cells(cells) {}
    };

}

std::string Day04Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<std::vector<Item>> cells;
    while (std::getline(ss, line, '\n')) {
        std::vector<Item> row;
        for (char c : line) {
            Position postion(row.size(), cells.size());
            Item item(c, postion);
            row.push_back(item);
        }
        cells.push_back(row);
    }

    Grid grid(cells);
    int accessible_papers = 0;

    for (int y = 0; y < (int)cells.size(); y++) {
        for (int x = 0; x < (int)cells[y].size(); x++) {
            Position pos(x, y);
            Item item = grid.get(pos);

            if (item.is_paper()) {
                int npapers = 0;
                for (Item neighbor : grid.neighbours(item)) {
                    if (neighbor.is_paper()) {
                        npapers++;
                    }
                }

                if (npapers < 4) {
                    accessible_papers++;
                }
            }
        }
    }

    return std::to_string(accessible_papers);
}

std::string Day04Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<std::vector<Item>> cells;
    while (std::getline(ss, line, '\n')) {
        std::vector<Item> row;
        for (char c : line) {
            Position postion(row.size(), cells.size());
            Item item(c, postion);
            row.push_back(item);
        }
        cells.push_back(row);
    }

    Grid grid(cells);
    int total_accessible_papers = 0;
    int curr_accessible_papers = -1;
    std::vector<Item> to_remove;

    while (curr_accessible_papers != 0) {
        for (auto p : to_remove) {
            grid.remove(p.position);
        }

        curr_accessible_papers = 0;
        to_remove.clear();

        for (int y = 0; y < (int)cells.size(); y++) {
            for (int x = 0; x < (int)cells[y].size(); x++) {
                Position pos(x, y);
                Item item = grid.get(pos);

                if (item.is_paper()) {
                    int npapers = 0;
                    for (Item neighbor : grid.neighbours(item)) {
                        if (neighbor.is_paper()) {
                            npapers++;
                        }
                    }

                    if (npapers < 4) {
                        curr_accessible_papers++;
                        to_remove.push_back(item);
                    }
                }
            }
        }

        total_accessible_papers += curr_accessible_papers;
    }

    return std::to_string(total_accessible_papers);
}
