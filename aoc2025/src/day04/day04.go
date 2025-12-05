// Package day04 - advent of Code
package day04

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func checkNeighbords(coord complex64, diagram [][]rune, limitRolls int) bool {
	neighbours := []complex64{0 + 1i, 0 - 1i, 1 + 0i, -1 + 0i, 1 + 1i, -1 + 1i, 1 - 1i, -1 - 1i}
	x := real(coord)
	y := imag(coord)

	countRolls := 0
	for _, n := range neighbours {
		nx := int(x + real(n))
		ny := int(y + imag(n))

		if nx < 0 || nx >= len(diagram[0]) || ny < 0 || ny >= len(diagram) {
			continue
		}

		if diagram[ny][nx] == '@' {
			countRolls += 1
		}

	}

	return countRolls < limitRolls
}

func copyDiagram(original [][]rune) [][]rune {
	copied := make([][]rune, len(original))
	for i := range original {
		copied[i] = make([]rune, len(original[i]))
		copy(copied[i], original[i])
	}
	return copied
}

func PartOne() int {
	diagram, err := parser("src/day04/input.in")
	if err != nil {
		log.Fatal("Error parser:", err)
	}

	accessCount := 0

	for j, row := range diagram {
		for i := range row {
			if diagram[j][i] != '@' {
				continue
			}
			if checkNeighbords(complex(float32(i), float32(j)), diagram, 4) {
				accessCount += 1
			}
		}
	}

	return accessCount
}

func caculateRollsToRemove(diagram [][]rune) (int, [][]rune) {
	cDiagram := copyDiagram(diagram)
	numAccessRollsPaper := 0

	for j, row := range diagram {
		for i := range row {
			if diagram[j][i] != '@' {
				continue
			}
			if checkNeighbords(complex(float32(i), float32(j)), diagram, 4) {
				cDiagram[j][i] = 'x'
				numAccessRollsPaper += 1
			}
		}
	}

	return numAccessRollsPaper, cDiagram
}

func PartTwo() int {
	diagram, err := parser("src/day04/input.in")
	if err != nil {
		log.Fatal("Error parser:", err)
	}

	total := 0
	for {
		numAccessRollsPaper, newDiagram := caculateRollsToRemove(diagram)
		diagram = newDiagram
		if numAccessRollsPaper == 0 {
			break
		}
		total += numAccessRollsPaper

	}

	return total
}

func parser(filePath string) ([][]rune, error) {
	file, err := os.Open(filePath)
	result := [][]rune{}
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return result, err
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		row := []rune{}

		for cell := range strings.SplitSeq(line, "") {
			row = append(row, rune(cell[0]))
		}
		result = append(result, row)
	}

	return result, nil
}
