// Package day02 solves Advent of Code 2025 Day 1.
package day02

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	start int
	end   int
}

func checkTwiceSequence(num int) bool {
	s := strconv.Itoa(num)

	a, b := s[0:len(s)/2], s[len(s)/2:]

	return a == b
}

func checkAdvanceSequence(num int) bool {
	s := strconv.Itoa(num)

	l := len(s) / 2

	// fmt.Println("Sequence: ", num)
	for i := l; i > 0; i-- {
		ref := s[0:i]
		flag := true
		for j := i; j < len(s); j += i {
			if j+i > len(s) {
				flag = false
				break
			}
			other := s[j : j+i]
			// fmt.Printf("i: %d, j: %d\n", i, j)
			// fmt.Printf("compare ref: %s to other: %s\n", ref, other)
			if ref != other {
				flag = false
				break
			}
		}
		if flag {
			return true
		}
	}
	return false
}

func sum(nums []int) int {
	sum := 0

	for _, num := range nums {
		sum += num
	}

	return sum
}

func PartOne() int {
	ranges, err := parser("src/day02/input.in")
	if err != nil {
		log.Fatal(err)
	}

	results := []int{}

	for _, r := range ranges {
		for i := r.start; i <= r.end; i++ {
			if checkTwiceSequence(i) {
				results = append(results, i)
			}
		}
	}

	return sum(results)
}

func PartTwo() int {
	ranges, err := parser("src/day02/input.in")
	if err != nil {
		log.Fatal(err)
	}

	results := []int{}

	for _, r := range ranges {
		for i := r.start; i <= r.end; i++ {
			if checkAdvanceSequence(i) {
				results = append(results, i)
			}
		}
	}

	return sum(results)
}

func parser(filePath string) ([]Range, error) {
	result := []Range{}

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return result, err
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		for s := range strings.SplitSeq(line, ",") {
			parts := strings.SplitN(s, "-", 2)

			start, err := strconv.Atoi(parts[0])
			if err != nil {
				return []Range{}, err
			}

			end, err := strconv.Atoi(parts[1])
			if err != nil {
				return []Range{}, err
			}

			if start > end {
				return []Range{}, fmt.Errorf("wrong range, start %d bigger than end %d", start, end)
			}

			result = append(result, Range{start: start, end: end})
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Scanner err:", err)
		return []Range{}, err
	}

	return result, nil
}
