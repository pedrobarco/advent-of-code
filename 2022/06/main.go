package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	contains := func(arr []rune, el rune) bool {
		for _, a := range arr {
			if a == el {
				return true
			}
		}
		return false
	}

	fpos := func(text string, target int) int {
		marker := make([]rune, target)
		for i, c := range text {
			if len(marker) > 0 && contains(marker, c) {
				marker = marker[:0]
			}
			marker = append(marker, c)

			if len(marker) == target {
				return i + 1
			}
		}

		return -1
	}

	scanner.Scan()
	raw := scanner.Text()

	fmt.Println("1:", fpos(raw, 4))
	fmt.Println("2:", fpos(raw, 14))
}
