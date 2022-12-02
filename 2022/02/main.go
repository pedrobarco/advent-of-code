package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func index(arr []string, el string) int {
	for i, v := range arr {
		if v == el {
			return i
		}
	}
	return -1
}

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	m1 := []string{"A", "B", "C"}
	m2 := []string{"X", "Y", "Z"}

	const (
		win  int = 6
		draw int = 3
		lose int = 0
	)

	ans1 := 0
	ans2 := 0

	for scanner.Scan() {
		text := strings.Split(scanner.Text(), " ")
		t1 := text[0]
		t2 := text[1]
		p1 := index(m1, t1) + 1
		p2 := index(m2, t2) + 1

		// part 1
		out1 := p2
		if p2 == p1 {
			out1 += draw
		} else if p2-p1 == 1 || p2 == 1 && p1 == len(m1) {
			out1 += win
		}
		ans1 += out1

		// part 2
		out2 := lose
		if t2 == "Y" {
			out2 += draw + p1
		} else if t2 == "Z" {
			out2 += win
			if p1 == len(m1) {
				out2 += 1
			} else {
				out2 += p1 + 1
			}
		} else {
			if p1 == 1 {
				out2 += len(m1)
			} else {
				out2 += p1 - 1
			}
		}
		ans2 += out2
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
