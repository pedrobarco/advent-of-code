package day01

import (
	"bufio"
	"os"
	"sort"
	"strconv"

	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

type Day01 struct{}

var _ solver.Solver = (*Day01)(nil)

func (Day01) P1(input string) string {
	file, err := os.Open(input)
	if err != nil {
		panic(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	calories := make([]int, 0)
	sum := 0

	for scanner.Scan() {
		switch text := scanner.Text(); text {
		case "":
			calories = append(calories, sum)
			sum = 0
		default:
			num, err := strconv.Atoi(text)
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
	file, err := os.Open(input)
	if err != nil {
		panic(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	calories := make([]int, 0)
	sum := 0

	for scanner.Scan() {
		switch text := scanner.Text(); text {
		case "":
			calories = append(calories, sum)
			sum = 0
		default:
			num, err := strconv.Atoi(text)
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
