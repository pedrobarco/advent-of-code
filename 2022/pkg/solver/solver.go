package solver

import (
	"fmt"
	"os"
	"strings"
)

type Solver interface {
	P1(input string) string
	P2(input string) string
}

func NthInput(base string, n int) string {
	file := fmt.Sprintf("%s/day%s.in", base, nthDay(n))
	return parseInput(file)
}

func NthPiOutput(base string, n, i int) string {
	file := fmt.Sprintf("%s/day%sp%d.out", base, nthDay(n), i)
	return parseInput(file)
}

func parseInput(fp string) string {
	raw, err := os.ReadFile(fp)
	if err != nil {
		panic(err)
	}

	input := strings.TrimSuffix(string(raw), "\n")
	return input
}

func nthDay(n int) string {
	return fmt.Sprintf("%02d", n)
}
