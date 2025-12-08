// Package day05 - advent of Code
package day05

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

func combineRanges(ranges []Range) ([]Range, int) {
	newRanges := []Range{}
	numUpdateRange := 0

	hasUpdatedRange := false
	for _, r := range ranges {
		if len(newRanges) == 0 {
			newRanges = append(newRanges, r)
			continue
		}

		hasUpdatedRange = false
		for j := 0; j < len(newRanges); j++ {
			nr := newRanges[j]

			if (r.start >= nr.start && r.start <= nr.end) ||
				(r.start < nr.start && r.end >= nr.start && r.end <= nr.end) ||
				(r.start <= nr.start && r.end >= nr.end) {
				newRanges[j].start = min(r.start, nr.start)
				newRanges[j].end = max(r.end, nr.end)
				numUpdateRange += 1
				hasUpdatedRange = true
				break
			}

		}

		if !hasUpdatedRange {
			newRanges = append(newRanges, r)
		}
	}

	return newRanges, numUpdateRange
}

func isFreshIngredient(ingredientID int, ranges []Range) bool {
	for _, r := range ranges {
		if ingredientID >= r.start && ingredientID <= r.end {
			return true
		}
	}
	return false
}

func PartOne() int {
	ranges, ingredientIDs, err := parser("src/day05/input.in")
	if err != nil {
		log.Fatal("Error parser:", err)
	}

	for {

		newRanges, numUpdates := combineRanges(ranges)

		ranges = newRanges

		if numUpdates == 0 {
			break
		}
	}

	freshIngredientsCounter := 0

	for _, ingredientID := range ingredientIDs {
		if isFreshIngredient(ingredientID, ranges) {
			freshIngredientsCounter += 1
		}
	}

	return freshIngredientsCounter
}

func PartTwo() int {
	ranges, _, err := parser("src/day05/input.in")
	if err != nil {
		log.Fatal("Error parser:", err)
	}

	for {

		newRanges, numUpdates := combineRanges(ranges)

		ranges = newRanges

		if numUpdates == 0 {
			break
		}
	}

	ingredientsFreshCounter := 0
	for _, r := range ranges {
		ingredientsFreshCounter += r.end - r.start + 1
	}
	return ingredientsFreshCounter
}

func parser(filePath string) ([]Range, []int, error) {
	ranges := []Range{}
	ingredientIDs := []int{}

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return ranges, ingredientIDs, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	flagRange := true
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		if len(line) == 0 {
			flagRange = false
			continue
		}
		if flagRange {
			parts := strings.Split(line, "-")
			start, err := strconv.Atoi(parts[0])
			if err != nil {
				return []Range{}, []int{}, err
			}
			end, err := strconv.Atoi(parts[1])
			if err != nil {
				return []Range{}, []int{}, err
			}

			ranges = append(ranges, Range{start: start, end: end})

		} else {
			ingredientID, err := strconv.Atoi(line)
			if err != nil {
				return []Range{}, []int{}, err
			}
			ingredientIDs = append(ingredientIDs, ingredientID)
		}

	}

	return ranges, ingredientIDs, nil
}
