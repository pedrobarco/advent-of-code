package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type node struct {
	val      int
	children []*node
	parent   *node
}

func (n *node) isList() bool {
	return n.val == -1
}

func NewNode(line string) *node {
	curr := &node{val: -1}

	for len(line) > 0 {
		switch {
		case line[0] == '[':
			new := &node{val: -1, parent: curr}
			curr.children = append(curr.children, new)
			curr = new
			line = line[1:]
		case line[0] == ']':
			curr = curr.parent
			line = line[1:]
			if len(line) > 0 && line[0] == ',' {
				line = line[1:]
			}
		default:
			f := func(c rune) bool {
				return c == ']' || c == ','
			}
			idx := strings.IndexFunc(line, f)
			num, _ := strconv.Atoi(line[:idx])
			curr.children = append(curr.children, &node{val: num})
			line = line[idx:]
			if line[0] == ',' {
				line = line[1:]
			}
		}
	}

	res := curr.children[0]
	res.parent = nil

	return res
}

func compare(left, right []*node) int {
	for i := 0; i < len(left) && i < len(right); i++ {
		l := left[i]
		r := right[i]

		if !l.isList() && !r.isList() {
			if l.val < r.val {
				return -1
			}
			if l.val > r.val {
				return 1
			}
		} else {
			lc := l.children
			rc := r.children

			if !l.isList() {
				lc = []*node{l}
			}

			if !r.isList() {
				rc = []*node{r}
			}

			if res := compare(lc, rc); res != 0 {
				return res
			}
		}
	}

	if len(left) < len(right) {
		return -1
	}

	if len(left) > len(right) {
		return 1
	}

	return 0
}

func less(left, right *node) bool {
	return compare(left.children, right.children) < 1
}

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	lines := make([]string, 0)
	for scanner.Scan() {
		raw := strings.Trim(scanner.Text(), " ")
		if raw == "" {
			continue
		}
		lines = append(lines, raw)
	}

	ans1 := 0
	ans2 := 1

	const (
		DivPacket1 = "[[2]]"
		DivPacket2 = "[[6]]"
	)

	packets := make([]*node, len(lines))
	for i := 0; i < len(lines); i += 2 {
		n1 := NewNode(lines[i])
		n2 := NewNode(lines[i+1])
		packets[i], packets[i+1] = n1, n2
		if less(n1, n2) {
			ans1 += (i / 2) + 1
		}
	}

	dp1 := NewNode(DivPacket1)
	dp2 := NewNode(DivPacket2)

	packets = append(packets, dp1)
	packets = append(packets, dp2)

	sort.Slice(packets, func(i, j int) bool {
		return less(packets[i], packets[j])
	})

	for i := range packets {
		if packets[i] == dp1 || packets[i] == dp2 {
			ans2 *= i + 1
		}
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
