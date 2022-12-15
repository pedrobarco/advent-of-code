package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var moves = [][2]int{{0, 1}, {-1, 1}, {1, 1}}

func drop(rocks map[[2]int]bool, maxy, void int, start [2]int) int {
	r := make(map[[2]int]bool, 0)
	for k, v := range rocks {
		r[k] = v
	}

	curr := start
	sand := 0

moves:
	for {
		for _, m := range moves {
			next := [2]int{curr[0] + m[0], curr[1] + m[1]}
			if !r[next] && next[1] <= void {
				curr = next
				goto moves
			}
		}

		if curr == start {
			sand++
			break
		}

		if curr[1] == void && maxy < void {
			curr[1] = curr[1] - 1
		}

		if curr[1] >= void {
			break
		}

		r[curr] = true
		sand++
		curr = start
	}
	return sand
}

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	atoi := func(str string) int {
		int, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}
		return int
	}

	coords := func(line string) [2]int {
		coords := strings.Split(strings.Trim(line, " "), ",")
		return [2]int{atoi(coords[0]), atoi(coords[1])}
	}

	rocks := make(map[[2]int]bool)
	void := 0
	for scanner.Scan() {
		raw := strings.Split(scanner.Text(), "->")
		for i := 1; i < len(raw); i++ {
			from := coords(raw[i-1])
			to := coords(raw[i])

			yinc := false
			var min, max int
			if from[0] == to[0] && to[1] > from[1] {
				yinc = true
				min, max = from[1], to[1]
			} else if from[0] == to[0] && to[1] < from[1] {
				yinc = true
				min, max = to[1], from[1]
			} else if from[1] == to[1] && to[0] > from[0] {
				min, max = from[0], to[0]
			} else if from[1] == to[1] && to[0] < from[0] {
				min, max = to[0], from[0]
			}

			for j := min; j <= max; j++ {
				var k [2]int
				if yinc {
					k = [2]int{to[0], j}
				} else {
					k = [2]int{j, to[1]}
				}
				rocks[k] = true

				if k[1] > void {
					void = k[1]
				}
			}
		}
	}

	Start := [2]int{500, 0}

	ans1 := drop(rocks, void, void, Start)
	ans2 := drop(rocks, void, void+2, Start)

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
