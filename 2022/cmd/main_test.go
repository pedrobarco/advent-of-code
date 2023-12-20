package main_test

import (
	"fmt"
	"os"
	"strings"
	"testing"

	"github.com/pedrobarco/advent-of-code/2022/internal/day01"
	"github.com/pedrobarco/advent-of-code/2022/internal/day02"
	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

func TestSolvers(t *testing.T) {
	exs := []solver.Solver{
		day01.Day01{},
		day02.Day02{},
	}

	for i, ex := range exs {
		day := fmt.Sprintf("%02d", i+1)
		path := fmt.Sprintf("../test_data/day%s.in", day)
		content, err := os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}

		input := string(content)

		path = fmt.Sprintf("../test_data/day%sp1.out", day)
		content, err = os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}

		got := ex.P1(input)
		want := strings.TrimSuffix(string(content), "\n")
		if got != want {
			t.Errorf("day%sp1: got %q, wanted %q", day, got, want)
		}

		path = fmt.Sprintf("../test_data/day%sp2.out", day)
		content, err = os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}

		got = ex.P2(input)
		want = strings.TrimSuffix(string(content), "\n")
		if got != want {
			t.Errorf("day%sp2: got %q, wanted %q", day, got, want)
		}
	}
}
