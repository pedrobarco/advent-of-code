package main

import (
	"bufio"
	"fmt"
	"os"
)

const (
	StartingPos   = 'S'
	BestSignalPos = 'E'
)

var moves = [4][2]int{
	{0, 1},
	{0, -1},
	{1, 0},
	{-1, 0},
}

type node struct {
	x, y  int
	elev  rune
	steps int
}

func (n *node) Distance(t *node) int {
	srune := n.elev
	trune := t.elev
	if srune == StartingPos {
		srune = 'a'
	} else if srune == BestSignalPos {
		srune = 'z'
	}
	if trune == StartingPos {
		trune = 'a'
	} else if trune == BestSignalPos {
		trune = 'z'
	}
	return int(trune - srune)
}

func parseGrid() (grid [][]*node, starters []*node, source, target *node) {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	y := 0
	for scanner.Scan() {
		raw := scanner.Text()
		row := make([]*node, len(raw))
		for x, c := range raw {
			n := &node{x, y, c, 0}
			row[x] = n
			if n.elev == StartingPos {
				source = n
			}
			if n.elev == StartingPos || n.elev == 'a' {
				starters = append(starters, n)
			}
			if n.elev == BestSignalPos {
				target = n
			}
		}
		grid = append(grid, row)
		y++
	}
	return grid, starters, source, target
}

func findPathSteps(grid [][]*node, s, t *node, reverse bool) int {
	open := make([]*node, 0)
	closed := make(map[*node]bool, 0)
	open = append(open, s)
	for len(open) > 0 {
		curr := open[0]
		open = open[1:]
		if closed[curr] {
			continue
		}
		closed[curr] = true

		if grid[curr.y][curr.x].elev == t.elev {
			return curr.steps
		}

		for _, m := range moves {
			x, y := curr.x+m[0], curr.y+m[1]
			if y >= 0 && y < len(grid) && x >= 0 && x < len(grid[y]) {
				next := grid[y][x]
				dist := curr.Distance(next)
				if (!reverse && dist <= 1) || (reverse && dist >= -1) {
					next.steps = curr.steps + 1
					open = append(open, next)
				}
			}
		}
	}
	return -1
}

func main() {
	grid1, starters, source, target := parseGrid()

	ans1 := findPathSteps(grid1, source, target, false)
	ans2 := 10000000

	for i := range starters {
		g, st, _, t := parseGrid()
		dist := findPathSteps(g, t, st[i], true)
		if dist < ans2 {
			ans2 = dist
		}
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
