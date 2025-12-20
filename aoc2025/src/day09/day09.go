// Package day09 - advent of Code
package day09

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type Point struct {
	x, y int
}

type Grid struct {
	width, height int
	vertices      []Point
	rows          map[int][]Point
	columns       map[int][]Point
}

type Distance struct {
	left, right int
}

func absInt(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func PartOne() int {
	grid, err := parser("src/day09/input.in")
	if err != nil {
		log.Fatalf("Failed to parse %v\n", err)
	}

	largestArea := 0
	for i := range grid.vertices {
		for j := i + 1; j < len(grid.vertices); j++ {
			base := absInt(grid.vertices[i].x-grid.vertices[j].x) + 1
			height := absInt(grid.vertices[i].y-grid.vertices[j].y) + 1

			area := base * height

			if largestArea < area {
				largestArea = area
			}

		}
	}
	return largestArea
}

func PartTwo() int {
	grid, err := parser("src/day09/input.in")
	if err != nil {
		log.Fatalf("Failed to parse %+v\n", err)
	}

	rowMap := make(map[int][]Point)

	for _, v := range grid.vertices {
		for _, p := range grid.columns[v.x] {
			if v != p {

				miny := min(v.y, p.y)
				maxy := max(v.y, p.y)

				for y := miny; y <= maxy; y++ {
					rowMap[y] = append(rowMap[y], Point{v.x, y})
				}

			}
		}
		for _, p := range grid.rows[v.y] {
			if v != p {
				minx := min(v.x, p.x)
				maxx := max(v.x, p.x)

				for x := minx; x <= maxx; x++ {
					rowMap[v.y] = append(rowMap[v.y], Point{x, v.y})
				}
			}
		}
	}

	rowRange := make(map[int]Distance)

	for k, v := range rowMap {
		right := 0
		left := math.MaxInt

		for _, p := range v {
			if p.x < left {
				left = p.x
			}
			if p.x > right {
				right = p.x
			}
		}

		rowRange[k] = Distance{left, right}
	}

	largestArea := 0
	for i := range grid.vertices {
		for j := i + 1; j < len(grid.vertices); j++ {
			base := absInt(grid.vertices[i].x-grid.vertices[j].x) + 1
			height := absInt(grid.vertices[i].y-grid.vertices[j].y) + 1

			area := base * height

			if largestArea < area && validRange(rowRange, grid.vertices[i], grid.vertices[j]) {
				largestArea = area
			}

		}
	}

	return largestArea
}

func validRange(rowRanges map[int]Distance, pa, pb Point) bool {
	miny := min(pa.y, pb.y)
	maxy := max(pa.y, pb.y)

	for i := miny; i <= maxy; i++ {

		distance := rowRanges[i]
		if pa.x < distance.left || pa.x > distance.right || pb.x < distance.left || pb.x > distance.right {
			return false
		}
	}
	return true
}

func parser(filePath string) (Grid, error) {
	grid := Grid{
		vertices: []Point{},
		rows:     make(map[int][]Point),
		columns:  make(map[int][]Point),
	}
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return grid, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	height := 0
	width := 0
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		parts := strings.Split(line, ",")
		x, err := strconv.Atoi(parts[0])
		if err != nil {
			log.Fatalf("Failed to convert %s to int\n", parts[0])
		}
		y, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Fatalf("Failed to convert %s to int\n", parts[1])
		}
		if width < x {
			width = x
		}

		if height < y {
			height = y
		}

		p := Point{x, y}
		grid.vertices = append(grid.vertices, p)
		grid.rows[y] = append(grid.rows[y], p)
		grid.columns[x] = append(grid.columns[x], p)
	}

	grid.width = width + 3
	grid.height = height + 2

	return grid, nil
}
