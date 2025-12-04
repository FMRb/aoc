// Package day03 - advent of Code
package day03

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func getLargestJoltage(numOn int, bank []int) int {
	result := 0
	temp := bank[:]
	for i := numOn; i > 0; i-- {
		// fmt.Printf("take temp %v\n", temp[:len(temp)-i+1])
		digit := slices.Max(temp[:len(temp)-i+1])
		idigit := slices.Index(temp, digit)
		temp = temp[idigit+1:]

		// fmt.Printf("digit %d, i %d, temp: %v\n", digit, i, temp)
		result += int(math.Pow10(i-1)) * digit
	}
	// ldigit := slices.Max(bank[:len(bank)-1])

	// lindex := slices.Index(bank, ldigit)
	// rdigit := slices.Max(bank[lindex+1:])
	// fmt.Println("RESULT ", result)

	return result
}

func PartOne() int {
	banks, err := parser("src/day03/example.in")
	if err != nil {
		log.Fatal("Error parser:", err)
	}

	result := 0
	for _, bank := range banks {
		result += getLargestJoltage(2, bank)
	}
	return result
}

func PartTwo() int {
	banks, err := parser("src/day03/input.in")
	if err != nil {
		log.Fatal("Error parser:", err)
	}

	result := 0
	for _, bank := range banks {
		result += getLargestJoltage(12, bank)
	}
	return result
}

func parser(filePath string) ([][]int, error) {
	file, err := os.Open(filePath)
	result := [][]int{}
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return result, err
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		batteries := []int{}

		for digit := range strings.SplitSeq(line, "") {
			battery, err := strconv.Atoi(digit)
			if err != nil {
				return [][]int{}, err
			}
			batteries = append(batteries, battery)
		}
		result = append(result, batteries)
	}

	return result, nil
}
