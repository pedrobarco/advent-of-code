package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	calories := make([]int, 0)
	sum := 0

	for scanner.Scan() {
		switch text := scanner.Text(); text {
		case "":
			calories = append(calories, sum)
			sum = 0
		default:
			num, err := strconv.Atoi(text)
			if err != nil {
				panic(err)
			}
			sum += num
		}
	}

	sort.Sort(sort.Reverse(sort.IntSlice(calories)))
	ans1 := calories[0]
	ans2 := 0

	for _, c := range calories[:3] {
		ans2 += c
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
