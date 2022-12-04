package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
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

	ans1 := 0
	ans2 := 0

	atoi := func(str string) int {
		int, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}
		return int
	}

	for scanner.Scan() {
		raw := strings.Split(scanner.Text(), ",")
		e1, e2 := strings.Split(raw[0], "-"), strings.Split(raw[1], "-")

		min1, max1 := atoi(e1[0]), atoi(e1[1])
		min2, max2 := atoi(e2[0]), atoi(e2[1])

		// part 1
		if (min1 >= min2 && max1 <= max2) ||
			(min2 >= min1 && max2 <= max1) {
			ans1++
		}

		// part 2
		if (min1 >= min2 && min1 <= max2) ||
			(max1 >= min2 && max1 <= max2) ||
			(min2 >= min1 && min2 <= max1) ||
			(max2 >= max1 && max2 <= max1) {
			ans2++
		}
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
