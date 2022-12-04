package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	const (
		loweroffset = int(rune('a')) - 1
		upperoffset = int(rune('A')) - 1 - 26
	)

	priority := func(char rune) int {
		ichar := int(char)
		if ichar > 96 {
			return ichar - loweroffset
		}
		return ichar - upperoffset
	}

	ans1 := 0
	ans2 := 0

	i := 1
	group := make([]string, 0)

	for scanner.Scan() {
		raw := scanner.Text()
		c1 := raw[:len(raw)/2]
		c2 := raw[len(raw)/2:]

		// part 1
		for _, s1 := range c1 {
			if strings.Contains(c2, string(s1)) {
				ans1 += priority(s1)
				break
			}
		}

		// part 2
		group = append(group, raw)
		if i%3 == 0 {
			for _, c := range raw {
				if strings.Contains(group[0], string(c)) &&
					strings.Contains(group[1], string(c)) {
					ans2 += priority(c)
					break
				}
			}
			group = make([]string, 0)
		}
		i++
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
