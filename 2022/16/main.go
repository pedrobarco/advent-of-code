package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type valve struct {
	name    string
	rate    uint16
	tunnels string
}

func floydWarshall(valves []*valve) map[*valve]map[*valve]uint16 {
	graph := make(map[*valve]map[*valve]uint16)
	for _, v1 := range valves {
		graph[v1] = make(map[*valve]uint16)
		for _, v2 := range valves {
			if v1 == v2 {
				graph[v1][v2] = 0
			} else if strings.Contains(v1.tunnels, v2.name) {
				graph[v1][v2] = 1
			} else {
				graph[v1][v2] = 0xff
			}
		}
	}

	for _, k := range valves {
		for _, i := range valves {
			for _, j := range valves {
				if graph[i][j] > graph[i][k]+graph[k][j] {
					graph[i][j] = graph[i][k] + graph[k][j]
				}
			}
		}
	}

	return graph
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

	re := regexp.MustCompile(`Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)`)
	valves := make([]*valve, 0)
	for scanner.Scan() {
		raw := scanner.Text()
		res := re.FindStringSubmatch(raw)
		v := res[1]
		r := uint16(atoi(res[2]))
		tt := res[3]
		valves = append(valves, &valve{name: v, rate: r, tunnels: tt})
	}

	graph := floydWarshall(valves)

	// pick valves with flow and starting point
	var worthit []*valve
	for _, v := range valves {
		if v.rate > 0 || v.name == "AA" {
			worthit = append(worthit, v)
		}
	}

	// assign bits
	bitfield := make(map[*valve]uint16)
	for idx, v := range worthit {
		bitfield[v] = 1 << idx
	}

	// find start
	var start uint16
	for _, v := range worthit {
		if v.name == "AA" {
			start = bitfield[v]
			break
		}
	}

	// create slice for fast edge lookup
	bitgraphsl := make([]uint16, 0xffff)
	for _, v1 := range worthit {
		for _, v2 := range worthit {
			bitgraphsl[bitfield[v1]|bitfield[v2]] = graph[v1][v2]
		}
	}

	// create slice for fast node lookup
	worthbitsl := make([][2]uint16, len(worthit))
	for idx, v := range worthit {
		worthbitsl[idx] = [2]uint16{bitfield[v], v.rate}
	}

	// part 1
	var dfs func(target, pressure, minute, on, node uint16) uint16
	dfs = func(target, pressure, minute, on, node uint16) uint16 {
		max := pressure
		for _, w := range worthbitsl {
			if node == w[0] || w[0] == start || w[0]&on != 0 {
				continue
			}
			l := bitgraphsl[node|w[0]] + 1
			if minute+l > target {
				continue
			}
			if next := dfs(target, pressure+(target-minute-l)*w[1], minute+l, on|w[0], w[0]); next > max {
				max = next
			}
		}
		return max
	}

	// part 2
	var dfspaths func(target, pressure, minute, on, node, path uint16) [][2]uint16
	dfspaths = func(target, pressure, minute, on, node, path uint16) [][2]uint16 {
		paths := [][2]uint16{{pressure, path}}
		for _, w := range worthbitsl {
			if w[0] == node || w[0] == start || w[0]&on != 0 {
				continue
			}
			l := bitgraphsl[node|w[0]] + 1
			if minute+l > target {
				continue
			}
			paths = append(paths, dfspaths(target, pressure+(target-minute-l)*w[1], minute+l, on|w[0], w[0], path|w[0])...)
		}
		return paths
	}

	ans1 := dfs(30, 0, 0, 0, start)
	allpaths := dfspaths(26, 0, 0, 0, start, 0)

	// reduce paths (presumably, both paths are at least half of part 1)
	var trimpaths [][2]uint16
	for _, p := range allpaths {
		if p[0] > ans1/2 {
			trimpaths = append(trimpaths, p)
		}
	}

	// compare all paths to find max
	var max uint16 = 0
	for idx := 0; idx < len(trimpaths); idx += 1 {
		for jdx := idx + 1; jdx < len(trimpaths); jdx += 1 {
			if trimpaths[idx][1]&trimpaths[jdx][1] != 0 {
				continue
			}
			if m := trimpaths[idx][0] + trimpaths[jdx][0]; m > max {
				max = m
			}
		}
	}

	ans2 := max

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
