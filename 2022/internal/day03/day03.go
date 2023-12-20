package day03

import (
	"strconv"
	"strings"

	"github.com/pedrobarco/advent-of-code/2022/pkg/solver"
)

const (
	loweroffset = int(rune('a')) - 1
	upperoffset = int(rune('A')) - 1 - 26
)

type Day03 struct{}

var _ solver.Solver = (*Day03)(nil)

func (Day03) P1(input string) string {
	ans := 0

	for _, line := range strings.Split(input, "\n") {
		c1 := line[:len(line)/2]
		c2 := line[len(line)/2:]

		for _, s1 := range c1 {
			if strings.Contains(c2, string(s1)) {
				ans += priority(s1)
				break
			}
		}
	}

	return strconv.Itoa(ans)
}

func (Day03) P2(input string) string {
	ans := 0

	i := 1
	group := make([]string, 0)

	for _, line := range strings.Split(input, "\n") {
		group = append(group, line)
		if i%3 == 0 {
			for _, c := range line {
				if strings.Contains(group[0], string(c)) &&
					strings.Contains(group[1], string(c)) {
					ans += priority(c)
					break
				}
			}
			group = make([]string, 0)
		}
		i++
	}

	return strconv.Itoa(ans)
}

func priority(char rune) int {
	ichar := int(char)
	if ichar > 96 {
		return ichar - loweroffset
	}
	return ichar - upperoffset
}
