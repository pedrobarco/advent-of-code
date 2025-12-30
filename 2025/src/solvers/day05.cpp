#include "day05.h"
#include <sstream>
#include <string>
#include <vector>

int Day05Solver::kDay() {
    return 5;
}

class FreshRange {
    public:
        long start;
        long end;

        bool is_fresh(long v) {
            return v >= start && v <= end;
        }

        FreshRange(long s, long e) : start(s), end(e) {}
};

std::string Day05Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<FreshRange> ranges;
    bool in_ranges(true);
    int fresh_items = 0;

    while(std::getline(ss, line, '\n')) {
        if (line.empty()) {
            in_ranges = false;
            continue;
        }

        if (in_ranges) {
            size_t dash_pos = line.find('-');
            long start = std::stol(line.substr(0, dash_pos));
            long end = std::stol(line.substr(dash_pos + 1));
            ranges.emplace_back(start, end);
            continue;
        }

        long v = std::stol(line);
        for (FreshRange range : ranges) {
            if (range.is_fresh(v)) {
                fresh_items++;
                break;
            }
        }
    }

    return std::to_string(fresh_items);
}

std::string Day05Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<FreshRange> ranges;

    while(std::getline(ss, line, '\n')) {
        if (line.empty()) {
            break;
        }

        size_t dash_pos = line.find('-');
        long start = std::stol(line.substr(0, dash_pos));
        long end = std::stol(line.substr(dash_pos + 1));
        FreshRange curr(start, end);
        ranges.push_back(curr);
    }

    std::sort(ranges.begin(), ranges.end(),
        [](const FreshRange& a, const FreshRange& b) {
            return a.start < b.start;
        });

    long fresh_items = 0;
    long curr_start = ranges[0].start;
    long curr_end = ranges[0].end;
    for (size_t i = 1; i < ranges.size(); ++i) {
        if (ranges[i].start <= curr_end + 1) {
            // overlap - extend current range
            curr_end = std::max(curr_end, ranges[i].end);
        } else {
            // no overlap - add current range and start new
            fresh_items += (curr_end - curr_start + 1);
            curr_start = ranges[i].start;
            curr_end   = ranges[i].end;
        }
    }

    fresh_items += (curr_end - curr_start + 1);

    return std::to_string(fresh_items);
}

