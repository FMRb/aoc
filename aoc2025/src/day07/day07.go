// Package day07 - advent of Code
package day07

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strings"
)

func PartOne() int {
	diagram, err := parser("src/day07/input.in")
	if err != nil {
		log.Fatal("Parse error", err)
	}

	result := 0
	for i := 1; i < len(diagram); i++ {
		newDiagram, numHitSplitter := processBeam(i, i-1, diagram)
		diagram = newDiagram
		result += numHitSplitter
		// fmt.Printf("####### Iteration %d #######\n", i)
		// printDiagram(diagram)
		// fmt.Println("--------------------")
	}
	return result
}

type Point struct {
	i, j int
}

func PartTwo() int {
	diagram, err := parser("src/day07/input.in")
	if err != nil {
		log.Fatal("Parse error", err)
	}

	cache := make(map[Point]int)
	result := processTimeline(1, 0, diagram, cache)
	return result
}

func printDiagram(diagram [][]rune) {
	for _, d := range diagram {
		fmt.Println(string(d))
	}
}

func processTimeline(i, iminus int, diagram [][]rune, cache map[Point]int) int {
	if len(diagram) == i {
		return 1
	}
	if slices.Contains(diagram[iminus], 'S') {
		nb := slices.Index(diagram[iminus], 'S')
		diagram[i][nb] = '|'
		return processTimeline(i+1, i, diagram, cache)
	}

	// fmt.Printf("####### Iteration %d #######\n", i)
	// printDiagram(diagram)
	// fmt.Println("--------------------")

	result := 0
	for j := 0; j < len(diagram[iminus]); j++ {
		if diagram[iminus][j] == '|' {
			switch cell := diagram[i][j]; cell {
			case '.':
				diagram[i][j] = '|'
				result += processTimeline(i+1, i, diagram, cache)
				diagram[i][j] = '.'
			case '^':
				if val, exists := cache[Point{i, j}]; exists {
					result += val
					continue
				}
				if j > 0 {
					original := diagram[i][j-1]
					diagram[i][j-1] = '|'
					result += processTimeline(i+1, i, diagram, cache)
					diagram[i][j-1] = original
				}
				if j < len(diagram[i])-1 {
					original := diagram[i][j+1]
					diagram[i][j+1] = '|'
					result += processTimeline(i+1, i, diagram, cache)
					diagram[i][j+1] = original
				}

				cache[Point{i, j}] = result
			}
		}
	}

	return result
}

func processBeam(i, iminus int, diagram [][]rune) ([][]rune, int) {
	hitSplitter := 0

	if slices.Contains(diagram[iminus], 'S') {
		nb := slices.Index(diagram[iminus], 'S')
		diagram[i][nb] = '|'
		return diagram, hitSplitter
	}

	for j := 0; j < len(diagram[iminus]); j++ {
		if diagram[iminus][j] == '|' {
			switch cell := diagram[i][j]; cell {
			case '.':
				diagram[i][j] = '|'
			case '^':
				hitSplitter += 1
				if j != 0 {
					diagram[i][j-1] = '|'
				}
				if j < len(diagram[i])-1 {
					diagram[i][j+1] = '|'
				}
			}
		}
	}

	return diagram, hitSplitter
}

func parser(filePath string) ([][]rune, error) {
	diagram := [][]rune{}
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return diagram, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	i := 0
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		diagram = append(diagram, []rune(line))
		i++
	}

	return diagram, nil
}
