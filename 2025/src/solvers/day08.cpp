#include "day08.h"
#include <algorithm>
#include <cmath>
#include <set>
#include <sstream>
#include <utility>
#include <vector>

int Day08Solver::kDay() {
    return 8;
}

namespace {

    struct Position {
        int x;
        int y;
        int z;
        Position(int x_, int y_, int z_) : x(x_), y(y_), z(z_) {}

        bool operator<(const Position& other) const {
            return std::tie(x, y, z) < std::tie(other.x, other.y, other.z);
        }

        double distance(const Position& other) const {
            double dx = static_cast<double>(other.x) - x;
            double dy = static_cast<double>(other.y) - y;
            double dz = static_cast<double>(other.z) - z;
            return std::sqrt(dx * dx + dy * dy + dz * dz);
        }
    };

    std::vector<std::set<Position>> make_n_closest_connections(const std::vector<std::pair<Position, Position>>& pairs, int n) {
        std::vector<std::set<Position>> circuits;

        for (int i = 0; i < n && i < static_cast<int>(pairs.size()); i++) {
            const auto& p = pairs[i];

            int first_idx = -1;
            int second_idx = -1;
            for (int j = 0; j < static_cast<int>(circuits.size()); ++j) {
                if (circuits[j].count(p.first))  first_idx = j;
                if (circuits[j].count(p.second)) second_idx = j;
                if (first_idx != -1 && second_idx != -1) break;
            }

            if (first_idx != -1 && first_idx == second_idx) {
                // duplicate, just skip
                continue;
            } else if (first_idx == -1 && second_idx != -1) {
                // second exists, add first to it
                circuits[second_idx].insert(p.first);
            } else if (first_idx != -1 && second_idx == -1) {
                // first exists, add second to it
                circuits[first_idx].insert(p.second);
            } else if (first_idx != -1 && second_idx != -1) {
                // both exists, merge circuits
                circuits[first_idx].insert(
                        circuits[second_idx].begin(),
                        circuits[second_idx].end()
                );
                circuits.erase(circuits.begin() + second_idx);
            } else {
                // neither exists, create new circuit
                circuits.push_back({p.first, p.second});
            }
        }

        return circuits;
    }

    bool is_circuit_complete(const std::set<Position>& circuit, int target) {
        return static_cast<int>(circuit.size()) == target;
    }

    std::pair<Position, Position> make_single_circuit(const std::vector<std::pair<Position, Position>>& pairs, int target) {
        std::vector<std::set<Position>> circuits;

        for (int i = 0; i < static_cast<int>(pairs.size()); i++) {
            const auto& p = pairs[i];

            int first_idx = -1;
            int second_idx = -1;
            for (int j = 0; j < static_cast<int>(circuits.size()); ++j) {
                if (circuits[j].count(p.first))  first_idx = j;
                if (circuits[j].count(p.second)) second_idx = j;
                if (first_idx != -1 && second_idx != -1) break;
            }

            if (first_idx != -1 && first_idx == second_idx) {
                // duplicate, just skip
                continue;
            } else if (first_idx == -1 && second_idx != -1) {
                // second exists, add first to it
                circuits[second_idx].insert(p.first);
            } else if (first_idx != -1 && second_idx == -1) {
                // first exists, add second to it
                circuits[first_idx].insert(p.second);

            } else if (first_idx != -1 && second_idx != -1) {
                // both exists, merge circuits
                circuits[first_idx].insert(
                        circuits[second_idx].begin(),
                        circuits[second_idx].end()
                );
                circuits.erase(circuits.begin() + second_idx);

                // Adjust first_idx if second_idx was erased before it
                if (second_idx < first_idx) {
                    first_idx--;
                }
                // Mark second_idx as invalid since the circuit was erased
                second_idx = -1;
            } else {
                // neither exists, create new circuit
                circuits.push_back({p.first, p.second});
            }

            bool first_complete = (first_idx != -1 && is_circuit_complete(circuits[first_idx], target));
            bool second_complete = (second_idx != -1 && is_circuit_complete(circuits[second_idx], target));
            bool complete = first_complete || second_complete;
            if (complete) return p;
        }

        throw std::runtime_error("unreachable");
    }

}

std::string Day08Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<Position> boxes;
    while(std::getline(ss, line)) {
        int x, y, z;
        sscanf(line.c_str(), "%d,%d,%d", &x, &y, &z);
        boxes.push_back(Position(x, y, z));
    }

    std::vector<std::pair<Position, Position>> pairs;
    for (size_t i = 0; i < boxes.size(); i++) {
        for (size_t j = i + 1; j < boxes.size(); j++) {
            pairs.push_back(std::make_pair(boxes[i], boxes[j]));
        }
    }

    std::sort(pairs.begin(), pairs.end(), [](const std::pair<Position, Position>& a, const std::pair<Position, Position>& b) {
        return a.first.distance(a.second) < b.first.distance(b.second);
    });

    int n = pairs.size() > 1000 ? 1000 : 10;
    auto circuits = make_n_closest_connections(pairs, n);
    std::sort(circuits.begin(), circuits.end(), [](const std::set<Position>& a, const std::set<Position>& b) {
        return a.size() > b.size();
    });

    long res = circuits[0].size() * circuits[1].size() * circuits[2].size();
    return std::to_string(res);
}

std::string Day08Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<Position> boxes;
    while(std::getline(ss, line)) {
        int x, y, z;
        sscanf(line.c_str(), "%d,%d,%d", &x, &y, &z);
        boxes.push_back(Position(x, y, z));
    }

    std::vector<std::pair<Position, Position>> pairs;
    for (size_t i = 0; i < boxes.size(); i++) {
        for (size_t j = i + 1; j < boxes.size(); j++) {
            pairs.push_back(std::make_pair(boxes[i], boxes[j]));
        }
    }

    std::sort(pairs.begin(), pairs.end(), [](const std::pair<Position, Position>& a, const std::pair<Position, Position>& b) {
        return a.first.distance(a.second) < b.first.distance(b.second);
    });

    auto pair = make_single_circuit(pairs, boxes.size());
    int res = pair.first.x * pair.second.x;
    return std::to_string(res);
}
