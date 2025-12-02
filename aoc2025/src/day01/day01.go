// Package day01 solves Advent of Code 2025 Day 1.
package day01

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Dial struct {
	dir   rune
	value int
}

func PartOne() {
	dialPos := 50
	countZeros := 0
	dials, err := parser()
	if err != nil {
		log.Fatal(err)
	}

	for _, d := range dials {
		if d.dir == 'L' {
			temp := dialPos - d.value
			if temp < 0 {
				temp += 100
			}
			dialPos = temp % 100
		} else {
			dialPos = (dialPos + d.value) % 100
		}

		if dialPos == 0 {
			countZeros += 1
		}

	}

	fmt.Println("Results:", countZeros)
}

func PartTwo() {
	dialPos := 50
	countZeros := 0
	dials, err := parser()
	if err != nil {
		panic("Invalid parser")
	}

	for _, d := range dials {
		times := d.value / 100
		value := d.value % 100
		boundCheck := 0
		wasZero := dialPos == 0
		if d.dir == 'L' {
			temp := dialPos - value
			boundCheck = temp
			if temp < 0 {
				temp += 100
			}
			dialPos = temp % 100
		} else {
			boundCheck = dialPos + value

			dialPos = (dialPos + value) % 100
		}

		// fmt.Printf("value: %d, boundCheck: %d, times: %d, countZeros: %d\n", d.value, boundCheck, times, countZeros)
		if dialPos == 0 {
			countZeros += 1
		} else if !wasZero && (boundCheck > 100 || boundCheck < 0) {
			countZeros += 1
		}
		countZeros += times

	}

	fmt.Println("Results:", countZeros)
}

func parser() ([]Dial, error) {
	filePath := "src/day01/input.in"

	result := []Dial{}

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return result, err
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		num, err := strconv.Atoi(line[1:])
		if err != nil {
			return []Dial{}, err
		}
		if line[0] == 'L' {
			result = append(result, Dial{dir: 'L', value: num})
		} else {
			result = append(result, Dial{dir: 'R', value: num})
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Scanner err:", err)
		return []Dial{}, err
	}

	return result, nil
}
