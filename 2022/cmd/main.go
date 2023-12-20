package main

import (
	"fmt"

	"github.com/pedrobarco/advent-of-code/2022/internal/day01"
	"github.com/pedrobarco/advent-of-code/2022/internal/day02"
	"github.com/pedrobarco/advent-of-code/2022/internal/day03"
	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

func main() {
	base := "./input"

	exs := []solver.Solver{
		day01.Day01{},
		day02.Day02{},
		day03.Day03{},
	}

	fmt.Println("############")
	fmt.Println("# AOC 2022 #")
	fmt.Println("############")

	for i, ex := range exs {
		day := i + 1
		input := solver.NthInput(base, day)
		fmt.Printf("Day %d\n", day)
		fmt.Printf("P1: %s\n", ex.P1(input))
		fmt.Printf("P2: %s\n", ex.P2(input))
	}
}
