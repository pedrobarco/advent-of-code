#include <cstdio>
#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <filesystem>
#include "reader.h"

std::filesystem::path const INPUTS_DIR = "inputs/";
std::filesystem::path const TESTS_DATA_DIR = "tests/data/";

std::string file_name(int day, int part) {
    char buf[3];

    std::snprintf(buf, sizeof(buf), "%02d", day);
    std::string padded_day = buf;

    std::snprintf(buf, sizeof(buf), "%02d", part);
    std::string padded_part = buf;

    return "d" + padded_day + "p" + padded_part;
}

std::string read_input(int day, int part) {
    auto name = file_name(day, part);
    auto input = INPUTS_DIR / (name + ".in");
    std::ifstream file(input);
    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

std::string read_example_input(int day, int part) {
    auto name = file_name(day, part) + ".in";
    auto input = TESTS_DATA_DIR / name;
    std::ifstream file(input);
    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

std::string read_example_output(int day, int part) {
    auto name = file_name(day, part) + ".out";
    auto input = TESTS_DATA_DIR / name;
    std::ifstream file(input);
    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}
