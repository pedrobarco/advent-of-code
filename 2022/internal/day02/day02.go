package day02

import (
	"strconv"
	"strings"

	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

const (
	win  int = 6
	draw int = 3
	lose int = 0
)

type Day02 struct{}

var _ solver.Solver = (*Day02)(nil)

func (Day02) P1(input string) string {
	m1 := []string{"A", "B", "C"}
	m2 := []string{"X", "Y", "Z"}
	ans := 0

	for _, line := range strings.Split(input, "\n") {
		text := strings.Split(line, " ")
		t1 := text[0]
		t2 := text[1]
		p1 := index(m1, t1) + 1
		p2 := index(m2, t2) + 1

		out := p2
		if p2 == p1 {
			out += draw
		} else if p2-p1 == 1 || p2 == 1 && p1 == len(m1) {
			out += win
		}
		ans += out
	}

	return strconv.Itoa(ans)
}

func (Day02) P2(input string) string {
	m1 := []string{"A", "B", "C"}
	ans := 0

	for _, line := range strings.Split(input, "\n") {
		text := strings.Split(line, " ")
		t1 := text[0]
		t2 := text[1]
		p1 := index(m1, t1) + 1

		out := lose
		if t2 == "Y" {
			out += draw + p1
		} else if t2 == "Z" {
			out += win
			if p1 == len(m1) {
				out += 1
			} else {
				out += p1 + 1
			}
		} else {
			if p1 == 1 {
				out += len(m1)
			} else {
				out += p1 - 1
			}
		}
		ans += out
	}

	return strconv.Itoa(ans)
}

func index(arr []string, el string) int {
	for i, v := range arr {
		if v == el {
			return i
		}
	}
	return -1
}
