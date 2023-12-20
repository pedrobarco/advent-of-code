package main_test

import (
	"testing"

	"github.com/pedrobarco/advent-of-code/2022/internal/day01"
	"github.com/pedrobarco/advent-of-code/2022/internal/day02"
	"github.com/pedrobarco/advent-of-code/2022/internal/day03"
	"github.com/pedrobarco/advent-of-code/2022/internal/day04"
	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

func TestSolvers(t *testing.T) {
	base := "../test_data"

	exs := []solver.Solver{
		day01.Day01{},
		day02.Day02{},
		day03.Day03{},
		day04.Day04{},
	}

	for i, ex := range exs {
		day := i + 1
		input := solver.NthInput(base, day)

		got := ex.P1(input)
		want := solver.NthPiOutput(base, day, 1)
		if got != want {
			t.Errorf("day %d part 1: got %q, wanted %q", day, got, want)
		}

		got = ex.P2(input)
		want = solver.NthPiOutput(base, day, 2)
		if got != want {
			t.Errorf("day %d part 2: got %q, wanted %q", day, got, want)
		}
	}
}
