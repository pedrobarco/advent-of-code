# Compiler settings
CXX = g++
CXXFLAGS = -std=c++20 -W -Wall -pedantic

# Core source files (shared between main app and tests)
CORE_SRCS = src/solver.cpp \
            src/reader.cpp \
            src/solvers/registry.cpp \
            $(wildcard src/solvers/day*.cpp)

# Main entry points
MAIN_SRC = src/main.cpp
TEST_SRC = tests/main.cpp

# Output binaries
BIN_DIR = bin
AOC_BIN = $(BIN_DIR)/aoc
TEST_BIN = $(BIN_DIR)/tests

# Default target
all: build

# Build main application
build: $(AOC_BIN)

$(AOC_BIN): $(CORE_SRCS) $(MAIN_SRC)
	$(CXX) $(CXXFLAGS) -o $(AOC_BIN) $(CORE_SRCS) $(MAIN_SRC)

# Build and run main application
run: build
	./$(AOC_BIN) $(DAY)

# Build and run tests
test: $(TEST_BIN)
	./$(TEST_BIN)

$(TEST_BIN): $(CORE_SRCS) $(TEST_SRC)
	$(CXX) $(CXXFLAGS) -o $(TEST_BIN) $(CORE_SRCS) $(TEST_SRC)

# Clean built binaries
clean:
	rm -f $(AOC_BIN) $(TEST_BIN)

.PHONY: all build run test clean
