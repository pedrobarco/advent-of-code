package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	ans1 := ""
	ans2 := ""

	// move n times from src to dst
	re := regexp.MustCompile(`move (\d+) from (\d+) to (\d+)`)

	isMoving := false
	var cargo [][]rune
	var bigcargo [][]rune

	atoi := func(str string) int {
		num, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}
		return num
	}

	reverseRune := func(s []rune) []rune {
		res := make([]rune, len(s))
		prevPos, resPos := 0, len(s)
		for pos := range s {
			resPos -= pos - prevPos
			copy(res[resPos:], s[prevPos:pos])
			prevPos = pos
		}
		copy(res[0:], s[prevPos:])
		return res
	}

	for scanner.Scan() {
		raw := scanner.Text()
		if cargo == nil {
			cargo = make([][]rune, (len(raw)+1)/4)
		}

		if raw == "" {
			isMoving = true
			for i := range cargo {
				cargo[i] = reverseRune(cargo[i])
			}
			bigcargo = make([][]rune, len(cargo))
			copy(bigcargo, cargo)
			continue
		}

		// ignore cargo indexes
		if strings.HasPrefix(raw, " 1") {
			continue
		}

		if isMoving {
			match := re.FindStringSubmatch(raw)
			n := atoi(match[1])
			src := atoi(match[2]) - 1
			dst := atoi(match[3]) - 1

			// part 1
			for i := n; i > 0; i-- {
				x := cargo[src][len(cargo[src])-1]
				// pop from src
				cargo[src] = cargo[src][:len(cargo[src])-1]
				// push to dst
				cargo[dst] = append(cargo[dst], x)
			}

			// part 2
			tmpcrates := bigcargo[src][len(bigcargo[src])-n:]
			// pop from src
			bigcargo[src] = bigcargo[src][:len(bigcargo[src])-n]
			// push to dst
			bigcargo[dst] = append(bigcargo[dst], tmpcrates...)

		} else {
			for i, t := range raw {
				if (i-1)%4 == 0 && t != ' ' {
					ci := (i - 1) / 4
					cargo[ci] = append(cargo[ci], t)
				}
			}
		}
	}

	for _, stack := range cargo {
		ans1 += string(stack[len(stack)-1])
	}
	for _, stack := range bigcargo {
		ans2 += string(stack[len(stack)-1])
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
