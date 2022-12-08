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

	hmap := make([][]int, 0)

	for scanner.Scan() {
		raw := strings.Split(scanner.Text(), "")
		row := make([]int, len(raw))
		for i, c := range raw {
			row[i] = atoi(c)
		}
		hmap = append(hmap, row)
	}

	isVisible := func(matrix [][]int, ri, ci int) bool {
		row := matrix[ri]
		height := matrix[ri][ci]

		if ri == 0 || ri == len(hmap)-1 ||
			ci == 0 || ci == len(row)-1 {
			return true
		}

		// up: ri--
		for i := ri - 1; i >= 0; i-- {
			if matrix[i][ci] >= height {
				break
			}

			if i == 0 {
				return true
			}
		}

		// down
		for i := ri + 1; i < len(matrix); i++ {
			if matrix[i][ci] >= height {
				break
			}

			if i == len(matrix)-1 {
				return true
			}
		}

		// left: ci--
		for i := ci - 1; i >= 0; i-- {
			if matrix[ri][i] >= height {
				break
			}

			if i == 0 {
				return true
			}
		}

		// right
		for i := ci + 1; i < len(row); i++ {
			if matrix[ri][i] >= height {
				break
			}

			if i == len(row)-1 {
				return true
			}
		}

		return false
	}

	scenicScore := func(matrix [][]int, ri, ci int) int {
		row := matrix[ri]
		height := matrix[ri][ci]

		if ri == 0 || ri == len(hmap)-1 ||
			ci == 0 || ci == len(row)-1 {
			return 0
		}

		var (
			upscore    = 0
			downscore  = 0
			leftscore  = 0
			rightscore = 0
		)

		// up: ri--
		for i := ri - 1; i >= 0; i-- {
			upscore++
			if matrix[i][ci] >= height {
				break
			}
		}

		// down
		for i := ri + 1; i < len(matrix); i++ {
			downscore++
			if matrix[i][ci] >= height {
				break
			}
		}

		// left: ci--
		for i := ci - 1; i >= 0; i-- {
			leftscore++
			if matrix[ri][i] >= height {
				break
			}
		}

		// right
		for i := ci + 1; i < len(row); i++ {
			rightscore++
			if matrix[ri][i] >= height {
				break
			}
		}

		return upscore * downscore * leftscore * rightscore
	}

	for ri, row := range hmap {
		for ci := range row {
			if isVisible(hmap, ri, ci) {
				ans1++
			}
			score := scenicScore(hmap, ri, ci)
			if score > ans2 {
				ans2 = score
			}
		}
	}

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
