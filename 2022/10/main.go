package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	MinNth      = 20
	Nth         = 40
	SpriteWidth = 3
	CRTWidth    = 40
	CRTHeight   = 6
)

type cpu struct {
	x, cycles, strength int
	crt                 [][]bool
}

func (c *cpu) inc(n int) {
	for i := 0; i < n; i++ {
		c.cycles++
		c.updateStrength()
		c.updateCRT()
	}
}

func (c *cpu) updateStrength() {
	if c.cycles == MinNth || (c.cycles > Nth && c.cycles%Nth == MinNth) {
		c.strength += c.cycles * c.x
	}
}

func (c *cpu) updateCRT() {
	row := (c.cycles - 1) / CRTWidth
	col := (c.cycles - 1) % CRTWidth
	if col >= c.x-1 && col <= c.x+1 {
		c.crt[row][col] = true
	}
}

func (c *cpu) Noop() {
	c.inc(1)
}

func (c *cpu) AddX(x int) {
	c.inc(2)
	c.x += x
}

func (c *cpu) PrintCRT() string {
	str := strings.Builder{}
	for _, row := range c.crt {
		for _, el := range row {
			if el {
				str.WriteRune('#')
			} else {
				str.WriteRune('.')
			}
		}
		str.WriteRune('\n')
	}
	return str.String()
}

func NewCpu() *cpu {
	crt := make([][]bool, CRTHeight)
	for i := range crt {
		crt[i] = make([]bool, CRTWidth)
	}
	return &cpu{x: 1, crt: crt}
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

	acpu := NewCpu()
	for scanner.Scan() {
		raw := strings.Split(scanner.Text(), " ")
		cmd := raw[0]
		if cmd == "addx" {
			val := atoi(raw[1])
			acpu.AddX(val)
		} else if cmd == "noop" {
			acpu.Noop()
		}
	}

	ans1 := acpu.strength
	ans2 := acpu.PrintCRT()

	fmt.Println("1:", ans1)
	fmt.Println("2:\n", ans2)
}
