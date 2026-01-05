#include "day10.h"
#include <map>
#include <queue>
#include <set>
#include <sstream>
#include <vector>

int Day10Solver::kDay() {
    return 10;
}

namespace {

    struct IndicatorLight {
        std::vector<bool> states;

        IndicatorLight(std::string states_str) {
            for (std::size_t i = 0; i < states_str.size(); i++) {
                char c = states_str[i];
                if (c == '[' || c == ']') {
                    continue;
                }
                states.push_back(c == '#');
            }
        }

        bool operator<(const IndicatorLight &other) const {
            return states < other.states;
        }

        bool operator==(const IndicatorLight &other) const {
            return states == other.states;
        }
    };

    struct ButtonWiring {
        std::vector<int> toggles;

        ButtonWiring(std::string wiring_str) {
            std::string clean = wiring_str.substr(1, wiring_str.size() - 2);

            std::stringstream ss(clean);
            std::string num_str;
            while (std::getline(ss, num_str, ',')) {
                toggles.push_back(std::stoi(num_str));
            }
        }
    };

    struct JoltageRequirements {
        std::vector<int> targets;

        JoltageRequirements(std::string req_str) {
            std::string clean = req_str.substr(1, req_str.size() - 2);

            std::stringstream ss(clean);
            std::string num_str;
            while (std::getline(ss, num_str, ',')) {
                targets.push_back(std::stoi(num_str));
            }
        }
    };

    struct Machine {
        IndicatorLight indicator;
        IndicatorLight target;
        std::vector<ButtonWiring> wirings;
        JoltageRequirements joltage_requirements;

        Machine(IndicatorLight target_, std::vector<ButtonWiring> wirings_, JoltageRequirements joltage_) :
            // initialize the indicator to all off states
            indicator(std::format("[{}]", std::string(target_.states.size(), '.'))),
            target(target_),
            wirings(wirings_),
            joltage_requirements(joltage_) {
        }

        bool is_goal_state(const IndicatorLight& state) const {
            return state == target;
        }

        IndicatorLight apply_button(size_t button_index, const IndicatorLight& state) const {
            IndicatorLight next_state = state;
            for (int toggle_idx : wirings[button_index].toggles) {
                next_state.states[toggle_idx] = !next_state.states[toggle_idx];
            }
            return next_state;
        }
    };

    int fewest_presses(Machine& machine) {
        // Use BFS (Dijkstra with uniform cost) for shortest path
        std::queue<std::pair<IndicatorLight, int>> q;
        std::set<IndicatorLight> visited;

        q.push({machine.indicator, 0});
        visited.insert(machine.indicator);

        while (!q.empty()) {
            auto [current_indicator, presses] = q.front();
            q.pop();

            // Check if we've reached the goal
            if (machine.is_goal_state(current_indicator)) {
                return presses;
            }

            // Try pressing each button
            for (std::size_t i = 0; i < machine.wirings.size(); i++) {
                IndicatorLight next_indicator = machine.apply_button(i, current_indicator);

                // Add to queue if not visited
                if (visited.insert(next_indicator).second) {
                    q.push({next_indicator, presses + 1});
                }
            }
        }

        // No solution found
        return -1;
    }

    // Helper: Calculate which counters are affected (parity flipped) by a button combination
    std::vector<bool> calculate_parity_effect(
        const std::vector<int>& button_indices,
        const std::vector<ButtonWiring>& wirings,
        std::size_t num_counters) {
        
        std::vector<bool> parity(num_counters, false);
        for (int button_idx : button_indices) {
            for (int counter_idx : wirings[button_idx].toggles) {
                parity[counter_idx] = !parity[counter_idx];  // XOR operation
            }
        }
        return parity;
    }

    // Helper: Calculate the counter increments from a button combination
    std::vector<int> calculate_effects(
        const std::vector<int>& button_indices,
        const std::vector<ButtonWiring>& wirings,
        std::size_t num_counters) {
        
        std::vector<int> effects(num_counters, 0);
        for (int button_idx : button_indices) {
            for (int counter_idx : wirings[button_idx].toggles) {
                effects[counter_idx]++;
            }
        }
        return effects;
    }

    // Recursive bifurcation solver with memoization
    int solve_bifurcation(
        const std::vector<int>& targets,
        const std::map<std::vector<bool>, std::vector<std::vector<int>>>& parity_to_combos,
        const std::map<std::vector<int>, std::vector<int>>& combo_to_effects,
        std::map<std::vector<int>, int>& memo) {
        
        // Base case: all zeros
        bool all_zero = true;
        for (int val : targets) {
            if (val != 0) {
                all_zero = false;
                break;
            }
        }
        if (all_zero) {
            return 0;
        }

        // Check memoization
        auto memo_it = memo.find(targets);
        if (memo_it != memo.end()) {
            return memo_it->second;
        }

        // Calculate parity of current targets
        std::vector<bool> parity(targets.size());
        for (std::size_t i = 0; i < targets.size(); i++) {
            parity[i] = (targets[i] % 2) == 1;
        }

        // Try all button combinations that match this parity
        int min_presses = INT_MAX;
        
        auto parity_it = parity_to_combos.find(parity);
        if (parity_it != parity_to_combos.end()) {
            for (const auto& button_combo : parity_it->second) {
                // Get effects of this combination
                auto effects_it = combo_to_effects.find(button_combo);
                if (effects_it == combo_to_effects.end()) continue;
                
                const auto& effects = effects_it->second;
                
                // Calculate remaining after applying effects and halving
                std::vector<int> remaining(targets.size());
                bool valid = true;
                
                for (std::size_t i = 0; i < targets.size(); i++) {
                    int diff = targets[i] - effects[i];
                    if (diff < 0) {
                        valid = false;
                        break;
                    }
                    // After matching parity, the difference must be even to halve
                    if (diff % 2 != 0) {
                        valid = false;
                        break;
                    }
                    remaining[i] = diff / 2;
                }
                
                if (!valid) continue;
                
                // Recurse on halved remaining values
                int sub_result = solve_bifurcation(remaining, parity_to_combos, combo_to_effects, memo);
                
                if (sub_result != INT_MAX) {
                    int total_presses = (int)button_combo.size() + 2 * sub_result;
                    min_presses = std::min(min_presses, total_presses);
                }
            }
        }

        memo[targets] = min_presses;
        return min_presses;
    }

    int fewest_presses_joltage(Machine& machine) {
        std::size_t num_buttons = machine.wirings.size();
        std::size_t num_counters = machine.joltage_requirements.targets.size();
        
        // Build map: parity pattern -> list of button combinations that produce it
        std::map<std::vector<bool>, std::vector<std::vector<int>>> parity_to_combos;
        std::map<std::vector<int>, std::vector<int>> combo_to_effects;
        
        // Iterate through all possible button combinations (2^num_buttons)
        for (int mask = 0; mask < (1 << num_buttons); mask++) {
            // Build button combination from bitmask
            std::vector<int> combo;
            for (std::size_t i = 0; i < num_buttons; i++) {
                if (mask & (1 << i)) {
                    combo.push_back(i);
                }
            }
            
            // Calculate parity effect and counter effects
            std::vector<bool> parity = calculate_parity_effect(combo, machine.wirings, num_counters);
            std::vector<int> effects = calculate_effects(combo, machine.wirings, num_counters);
            
            // Add to maps
            parity_to_combos[parity].push_back(combo);
            combo_to_effects[combo] = effects;
        }
        
        // Solve using recursive bifurcation with memoization
        std::map<std::vector<int>, int> memo;
        int result = solve_bifurcation(
            machine.joltage_requirements.targets,
            parity_to_combos,
            combo_to_effects,
            memo
        );
        
        return result == INT_MAX ? -1 : result;
    }
}

std::string Day10Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    int res = 0;
    while(std::getline(ss, line)) {
        std::stringstream line_ss(line);
        std::string word;

        std::vector<std::string> words;
        while(std::getline(line_ss, word, ' ')) {
            words.push_back(word);
        }

        auto wirings_str = std::vector<std::string>(words.begin() + 1, words.end() - 1);

        IndicatorLight indicator(words[0]);
        std::vector<ButtonWiring> wirings;
        for (auto it = wirings_str.begin(); it != wirings_str.end(); ++it) {
            wirings.push_back(ButtonWiring(*it));
        }

        JoltageRequirements joltage(words.back());
        Machine machine(indicator, wirings, joltage);
        res += fewest_presses(machine);
    }

    return std::to_string(res);
}

std::string Day10Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    int res = 0;
    while(std::getline(ss, line)) {
        std::stringstream line_ss(line);
        std::string word;

        std::vector<std::string> words;
        while(std::getline(line_ss, word, ' ')) {
            words.push_back(word);
        }

        auto wirings_str = std::vector<std::string>(words.begin() + 1, words.end() - 1);

        IndicatorLight indicator(words[0]);
        std::vector<ButtonWiring> wirings;
        for (auto it = wirings_str.begin(); it != wirings_str.end(); ++it) {
            wirings.push_back(ButtonWiring(*it));
        }

        JoltageRequirements joltage(words.back());
        Machine machine(indicator, wirings, joltage);
        res += fewest_presses_joltage(machine);
    }

    return std::to_string(res);
}
