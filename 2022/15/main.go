package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

type sensor struct {
	pos    [2]int
	beacon [2]int
	dist   int
}

func (s sensor) colides(p [2]int) bool {
	isPos := p[0] == s.pos[0] && p[1] == s.pos[1]
	isBeacon := p[0] == s.beacon[0] && p[1] == s.beacon[1]
	return isPos || isBeacon
}

func dist(src, dst [2]int) int {
	dx := math.Abs(float64(dst[0] - src[0]))
	dy := math.Abs(float64(dst[1] - src[1]))
	return int(dx + dy)
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
		int, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}
		return int
	}

	re := regexp.MustCompile(`Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)`)
	network := make([]sensor, 0)
	xmin := 999999
	xmax := 0
	for scanner.Scan() {
		raw := scanner.Text()
		res := re.FindStringSubmatch(raw)
		pos := [2]int{atoi(res[1]), atoi(res[2])}
		beacon := [2]int{atoi(res[3]), atoi(res[4])}
		sensor := sensor{pos, beacon, dist(pos, beacon)}
		network = append(network, sensor)

		if x := pos[0] - sensor.dist; x < xmin {
			xmin = x
		}

		if x := pos[0] + sensor.dist; x > xmax {
			xmax = x
		}
	}

	// part1
	npos := func(network []sensor, xmin, xmax, y int) int {
		res := 0
		for x := xmin; x <= xmax; x++ {
			curr := [2]int{x, y}
			for _, s := range network {
				if dist(s.pos, curr) <= s.dist && !s.colides(curr) {
					res++
					break
				}
			}
		}
		return res
	}

	// part2
	tuningf := func(network []sensor, min, max int) int {
		curr := [2]int{}
		covering := sensor{}
		for {
			covered := false
			for _, s := range network {
				covered = s.dist >= dist(curr, s.pos)
				if covered {
					covering = s
					break
				}
			}

			if !covered {
				break
			}

			margin := covering.dist - dist(curr, covering.pos) + 1
			if curr[0]+margin > max {
				curr[0] = min
				curr[1]++
			} else {
				curr[0] += margin
			}
		}

		return curr[0]*max + curr[1]
	}

	ans1 := npos(network, xmin, xmax, 2000000)
	ans2 := tuningf(network, 0, 4000000)

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
