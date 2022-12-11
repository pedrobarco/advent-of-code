package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const (
	reliefFactor = 3
)

type monkey struct {
	items        []int
	op           func(item int) int
	div          int
	throwTrue    *monkey
	throwFalse   *monkey
	inspectTimes int
}

func (m *monkey) pushItem(item int) {
	m.items = append(m.items, item)
}

func (m *monkey) popItem() int {
	item := m.items[0]
	m.items = m.items[1:]
	return item
}

func (m *monkey) worryLevel(item int, gcd int) int {
	m.inspectTimes++
	if gcd != 0 {
		return m.op(item) % gcd
	}
	return m.op(item) / reliefFactor
}

func (m *monkey) test(gcd int) {
	item := m.worryLevel(m.popItem(), gcd)
	if item%m.div == 0 {
		m.throwTrue.pushItem(item)
	} else {
		m.throwFalse.pushItem(item)
	}
}

func (m *monkey) takeTurn(gcd int) {
	for {
		if len(m.items) == 0 {
			break
		}
		m.test(gcd)
	}
}

func NewMonkey(items []int) *monkey {
	return &monkey{items: items}
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
		num, err := strconv.Atoi(str)
		if err != nil {
			panic(err)
		}
		return num
	}

	const (
		ItemsPrefix     = "Starting items:"
		OperationPrefix = "Operation: new = old"
		TestPrefix      = "Test: divisible by"
		TestTruePrefix  = "If true: throw to monkey"
		TestFalsePrefix = "If false: throw to monkey"
	)

	monkeys := make([]*monkey, 0)
	tmpTestTrue := make([]int, 0)
	tmpTestFalse := make([]int, 0)
	for scanner.Scan() {
		raw := strings.Trim(scanner.Text(), " ")
		switch {
		case strings.HasPrefix(raw, ItemsPrefix):
			rawitems := strings.Split(strings.Trim(strings.Split(raw, ":")[1], " "), ", ")
			items := make([]int, 0)
			for _, item := range rawitems {
				items = append(items, atoi(item))
			}
			m := NewMonkey(items)
			monkeys = append(monkeys, m)
		case strings.HasPrefix(raw, OperationPrefix):
			m := monkeys[len(monkeys)-1]
			rawop := strings.Split(strings.Trim(strings.Split(raw, OperationPrefix)[1], " "), " ")
			m.op = func(item int) int {
				rawoperand := rawop[0]
				rawnum := rawop[1]

				var num int
				if rawnum == "old" {
					num = item
				} else {
					num = atoi(rawnum)
				}

				switch rawoperand {
				case "+":
					return item + num
				case "*":
					return item * num
				case "-":
					return item - num
				case "/":
					return item / num
				default:
					panic("no such operation")
				}
			}
		case strings.HasPrefix(raw, TestPrefix):
			m := monkeys[len(monkeys)-1]
			rawtest := strings.Trim(strings.Split(raw, TestPrefix)[1], " ")
			m.div = atoi(rawtest)
		case strings.HasPrefix(raw, TestTruePrefix):
			rawtest := strings.Trim(strings.Split(raw, TestTruePrefix)[1], " ")
			tmpTestTrue = append(tmpTestTrue, atoi(rawtest))
		case strings.HasPrefix(raw, TestFalsePrefix):
			rawtest := strings.Trim(strings.Split(raw, TestFalsePrefix)[1], " ")
			tmpTestFalse = append(tmpTestFalse, atoi(rawtest))
		default:
			continue
		}
	}

	for i, m := range monkeys {
		m.throwTrue = monkeys[tmpTestTrue[i]]
		m.throwFalse = monkeys[tmpTestFalse[i]]
	}

	gcd := func(monkeys []*monkey) int {
		pf := 1
		for _, m := range monkeys {
			pf *= m.div
		}
		return pf
	}

	getMonkeyBusiness := func(monkeys []*monkey, rounds, gcd int) int {
		for i := 1; i <= rounds; i++ {
			for _, m := range monkeys {
				m.takeTurn(gcd)
			}
		}

		activity := make([]int, len(monkeys))
		for _, m := range monkeys {
			activity = append(activity, m.inspectTimes)
		}
		sort.Sort(sort.Reverse(sort.IntSlice(activity)))
		return activity[0] * activity[1]
	}

	// ans1 := getMonkeyBusiness(monkeys, 20, 0)
	ans2 := getMonkeyBusiness(monkeys, 10000, gcd(monkeys))

	// fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
