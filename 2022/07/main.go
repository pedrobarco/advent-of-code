package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type dir struct {
	parent   *dir
	name     string
	filesize int
	children []*dir
}

func (d *dir) Size() int {
	size := d.filesize
	for _, c := range d.children {
		size += c.Size()
	}
	return size
}

func (d *dir) Move(path string) *dir {
	if path == ".." {
		return d.parent
	}
	for _, c := range d.children {
		if c.name == path {
			return c
		}
	}
	panic("no such path")
}

func (d *dir) AddFile(size int) {
	d.filesize += size
}

func (d *dir) AddDir(path string) {
	child := &dir{parent: d, name: path}
	d.children = append(d.children, child)
}

func getTotalSizeByLimit(d *dir, limit int) int {
	total := 0
	size := d.Size()
	if size <= limit {
		total += d.Size()
	}

	for _, c := range d.children {
		total += getTotalSizeByLimit(c, limit)
	}
	return total
}

func freeUpSpace(d *dir, target int) int {
	free := d.Size()
	for _, c := range d.children {
		cfree := freeUpSpace(c, target)
		if cfree >= target && cfree < free {
			free = cfree
		}
	}
	return free
}

func main() {
	filePath := os.Args[1]
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	var curr *dir
	var root *dir
	for scanner.Scan() {
		raw := strings.Split(scanner.Text(), " ")
		switch raw[0] {
		case "$":
			cmd := raw[1]
			if cmd == "cd" {
				target := raw[2]
				if curr == nil {
					curr = &dir{name: target}
					root = curr
				} else {
					curr = curr.Move(target)
				}
			}
		case "dir":
			path := raw[1]
			curr.AddDir(path)
		default:
			size, err := strconv.Atoi(raw[0])
			if err != nil {
				panic(err)
			}
			curr.AddFile(size)
		}
	}

	const (
		maxSize    = 100000
		fsSize     = 70000000
		updateSize = 30000000
	)

	ans1 := getTotalSizeByLimit(root, maxSize)
	ans2 := freeUpSpace(root, updateSize-(fsSize-root.Size()))

	fmt.Println("1:", ans1)
	fmt.Println("2:", ans2)
}
