package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type pos struct {
	x, y int
}

type rope struct {
	positions []*pos
	head      *rope
	tail      *rope
}

func (n *rope) getCurrentPos() *pos {
	return n.positions[len(n.positions)-1]
}

func (r *rope) getNextPos(direction string) *pos {
	curr := r.getCurrentPos()
	if r.head == nil {
		switch direction {
		case "U":
			return &pos{x: curr.x, y: curr.y + 1}
		case "D":
			return &pos{x: curr.x, y: curr.y - 1}
		case "R":
			return &pos{x: curr.x + 1, y: curr.y}
		case "L":
			return &pos{x: curr.x - 1, y: curr.y}
		default:
			panic("unknown direction")
		}
	}

	hpos := r.head.getCurrentPos()
	v := &pos{
		x: hpos.x - curr.x,
		y: hpos.y - curr.y,
	}
	dx := math.Abs(float64(v.x))
	dy := math.Abs(float64(v.y))

	// close enough
	if math.Max(dx, dy) < 2 {
		return curr
	}

	if dy == 2 {
		if v.y > 0 {
			v.y = v.y - 1
		} else {
			v.y = v.y + 1
		}
	}

	if dx == 2 {
		if v.x > 0 {
			v.x = v.x - 1
		} else {
			v.x = v.x + 1
		}
	}

	return &pos{
		x: curr.x + v.x,
		y: curr.y + v.y,
	}
}

func (n *rope) Move(direction string) {
	nextPos := n.getNextPos(direction)
	n.positions = append(n.positions, nextPos)
	if n.tail != nil {
		n.tail.Move(direction)
	}
}

func (n *rope) getTail() *rope {
	if n.tail == nil {
		return n
	}
	return n.tail.getTail()
}

func (n *rope) getTailUniqueNPos() int {
	distinct := make(map[string]bool)
	for _, p := range n.getTail().positions {
		k := fmt.Sprintf("(%d, %d)", p.x, p.y)
		distinct[k] = true
	}
	return len(distinct)
}

func NewRope(knots int) *rope {
	newKnot := func() *rope {
		return &rope{positions: []*pos{{0, 0}}}
	}
	r := newKnot()
	curr := r
	for i := 0; i < knots; i++ {
		tmp := newKnot()
		curr.tail, tmp.head = tmp, curr
		curr = tmp
	}
	return r
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
		num, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}
		return num
	}

	ans1 := 0
	ans2 := 0

	rope1 := NewRope(1)
	rope2 := NewRope(9)

	for scanner.Scan() {
		raw := strings.Split(scanner.Text(), " ")
		direction, steps := raw[0], atoi(raw[1])
		for i := 0; i < steps; i++ {
			rope1.Move(direction)
			rope2.Move(direction)
		}
	}

	ans1 = rope1.getTailUniqueNPos()
	ans2 = rope2.getTailUniqueNPos()

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
