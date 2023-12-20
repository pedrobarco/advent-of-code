package day01

import (
	"sort"
	"strconv"
	"strings"

	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

type Day01 struct{}

var _ solver.Solver = (*Day01)(nil)

func (Day01) P1(input string) string {
	calories := make([]int, 0)
	sum := 0

	for _, line := range strings.Split(input, "\n") {
		switch line {
		case "":
			calories = append(calories, sum)
			sum = 0
		default:
			num, err := strconv.Atoi(line)
			if err != nil {
				panic(err)
			}
			sum += num
		}
	}

	calories = append(calories, sum)

	sort.Sort(sort.Reverse(sort.IntSlice(calories)))
	res := calories[0]

	return strconv.Itoa(res)
}

func (Day01) P2(input string) string {
	calories := make([]int, 0)
	sum := 0

	for _, line := range strings.Split(input, "\n") {
		switch line {
		case "":
			calories = append(calories, sum)
			sum = 0
		default:
			num, err := strconv.Atoi(line)
			if err != nil {
				panic(err)
			}
			sum += num
		}
	}

	calories = append(calories, sum)

	sort.Sort(sort.Reverse(sort.IntSlice(calories)))
	res := 0

	for _, c := range calories[:3] {
		res += c
	}

	return strconv.Itoa(res)
}
