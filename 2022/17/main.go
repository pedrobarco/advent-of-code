package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type shape string

const (
	ShapeDash    shape = "-"
	ShapePlus    shape = "+"
	ShapeL       shape = "_|"
	ShapePipe    shape = "|"
	ShapeSquare  shape = "[]"
	Width        int   = 7
	MarginLeft   int   = 2
	MarginBottom int   = 3
)

type pos struct {
	x, y int
}

func (p pos) Add(r pos) pos {
	return pos{p.x + r.x, p.y + r.y}
}

func (p pos) Equals(r pos) bool {
	return p.x == r.x && p.y == r.y
}

type rock struct {
	location []pos
	shape    shape
}

type tower []*rock

func (t tower) String() string {
	str := strings.Builder{}
	ymax := t.Height()
	for y := ymax; y >= -1; y-- {
		isFloor := y == -1
		if isFloor {
			str.WriteRune('+')
		} else {
			str.WriteRune('|')
		}

		for x := 0; x < Width; x++ {
			if isFloor {
				str.WriteRune('-')
			} else {
				p := pos{x, y}
				if t.collides(nil, p) {
					str.WriteRune('#')
				} else {
					str.WriteRune('.')
				}
			}
		}

		if isFloor {
			str.WriteRune('+')
		} else {
			str.WriteRune('|')
		}

		str.WriteRune('\n')
	}
	return str.String()
}

func (t tower) Height() int {
	ymax := -1
	for _, r := range t {
		for _, p := range r.location {
			if p.y > ymax {
				ymax = p.y
			}
		}
	}
	return ymax
}

func (t tower) move(r *rock, vect pos) bool {
	l := make([]pos, 0)
	for _, p := range r.location {
		to := p.Add(vect)
		if t.collides(r, to) {
			return false
		}
		l = append(l, to)
	}
	r.location = l
	return true
}

func (t tower) collides(allow *rock, to pos) bool {
	if to.x < 0 || to.x >= Width || to.y < 0 {
		return true
	}
	for _, r := range t {
		if r == allow {
			continue
		}
		for _, p := range r.location {
			if p.Equals(to) {
				return true
			}
		}
	}
	return false
}

func (t tower) Drop(r *rock) bool {
	return t.move(r, pos{0, -1})
}

func (t tower) JetRight(r *rock) {
	_ = t.move(r, pos{1, 0})
}

func (t tower) JetLeft(r *rock) {
	_ = t.move(r, pos{-1, 0})
}

func NewRock(s shape, ymax int) *rock {
	r := &rock{shape: s}
	l := make([]pos, 0)
	start := pos{MarginLeft, ymax + 1 + MarginBottom}
	var tvects []pos
	switch s {
	case ShapeDash:
		tvects = []pos{{0, 0}, {1, 0}, {2, 0}, {3, 0}}
	case ShapePlus:
		// start with bottom block
		start.y++
		tvects = []pos{{0, 0}, {1, 0}, {2, 0}, {1, 1}, {1, -1}}
	case ShapeL:
		// start with bottom block
		tvects = []pos{{0, 0}, {1, 0}, {2, 0}, {2, 1}, {2, 2}}
	case ShapePipe:
		// start with bottom block
		tvects = []pos{{0, 0}, {0, 1}, {0, 2}, {0, 3}}
	case ShapeSquare:
		// start with bottom block
		tvects = []pos{{0, 0}, {0, 1}, {1, 0}, {1, 1}}
	default:
		panic("no such type")
	}
	for _, v := range tvects {
		l = append(l, start.Add(v))
	}
	r.location = l
	return r
}

func play(jets string, nrocks int) int {
	t := make(tower, 0)
	shapes := []shape{ShapeDash, ShapePlus, ShapeL, ShapePipe, ShapeSquare}
	ishape := 0
	ijets := 0
	var curr *rock
	for {
		if curr == nil {
			// spawn
			ymax := t.Height()
			if nrocks == len(t) {
				return ymax + 1
			}

			s := shapes[ishape%len(shapes)]
			ishape++
			curr = NewRock(s, ymax)
			t = append(t, curr)

		} else {
			// jet move
			jet := jets[ijets%len(jets)]
			ijets++

			if jet == '>' {
				t.JetRight(curr)
			} else {
				t.JetLeft(curr)
			}

			// drop
			if isResting := !t.Drop(curr); isResting {
				curr = nil
			}
		}
	}
}

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)
	scanner.Scan()

	jets := scanner.Text()

	ans1 := play(jets, 2022)
	ans2 := play(jets, 1000000000000)

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
