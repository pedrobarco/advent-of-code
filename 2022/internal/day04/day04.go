package day04

import (
	"strconv"
	"strings"

	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

type Day04 struct{}

var _ solver.Solver = (*Day04)(nil)

func (Day04) P1(input string) string {
	ans := 0

	for _, line := range strings.Split(input, "\n") {
		line := strings.Split(line, ",")
		e1, e2 := strings.Split(line[0], "-"), strings.Split(line[1], "-")

		min1, max1 := atoi(e1[0]), atoi(e1[1])
		min2, max2 := atoi(e2[0]), atoi(e2[1])

		if (min1 >= min2 && max1 <= max2) ||
			(min2 >= min1 && max2 <= max1) {
			ans++
		}
	}

	return strconv.Itoa(ans)
}

func (Day04) P2(input string) string {
	ans := 0

	for _, line := range strings.Split(input, "\n") {
		line := strings.Split(line, ",")
		e1, e2 := strings.Split(line[0], "-"), strings.Split(line[1], "-")

		min1, max1 := atoi(e1[0]), atoi(e1[1])
		min2, max2 := atoi(e2[0]), atoi(e2[1])

		if (min1 >= min2 && min1 <= max2) ||
			(max1 >= min2 && max1 <= max2) ||
			(min2 >= min1 && min2 <= max1) ||
			(max2 >= max1 && max2 <= max1) {
			ans++
		}
	}

	return strconv.Itoa(ans)
}

func atoi(str string) int {
	int, err := strconv.Atoi(str)
	if err != nil {
		panic(err)
	}
	return int
}
