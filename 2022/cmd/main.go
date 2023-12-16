package main

import (
	"fmt"

	"github.com/pedrobarco/advent-of-code/2022/internal/day01"
	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

func main() {
	exs := []solver.Solver{
		day01.Day01{},
	}

	fmt.Println("############")
	fmt.Println("# AOC 2022 #")
	fmt.Println("############")

	for i, ex := range exs {
		day := fmt.Sprintf("%02d", i+1)
		in := fmt.Sprintf("./input/day%s.in", day)

		fmt.Printf("Day %s\n", day)
		fmt.Printf("P1: %s\n", ex.P1(in))
		fmt.Printf("P2: %s\n", ex.P2(in))
	}
}
