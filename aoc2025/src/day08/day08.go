// Package day08 - advent of Code
package day08

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Box struct {
	x, y, z int
}

type BoxPair struct {
	box1, box2 Box
}

func PartOne() int {
	circuits := make(map[Box]int)
	circuitConnections := make(map[int][]Box)

	boxes, err := parser("src/day08/example.in")
	if err != nil {
		log.Fatal("Failed to parse")
	}

	distances := computeDistances(boxes)
	keys := make([]BoxPair, 0, len(distances))
	for k := range distances {
		keys = append(keys, k)
	}

	// Sort keys by their values
	sort.Slice(keys, func(i, j int) bool {
		return distances[keys[i]] < distances[keys[j]] // ascending order
	})

	circuitIndex := 1
	loops := 10

	for range loops {
		pairs := keys[:1][0]
		keys = keys[1:]

		indexOne, existsOne := circuits[pairs.box1]
		indexTwo, existsTwo := circuits[pairs.box2]

		if existsOne && existsTwo && indexOne == indexTwo {
			// fmt.Printf("b1 b2 both exits same index, %d %v %v\n", indexOne, pairs.box1, pairs.box2)
			continue
		}

		if existsOne && existsTwo {
			minIndex := min(indexOne, indexTwo)
			maxIndex := max(indexOne, indexTwo)

			// fmt.Printf("b1 b2 both exits, %d %v %v\n", minIndex, pairs.box1, pairs.box2)
			for _, b := range circuitConnections[maxIndex] {
				circuits[b] = minIndex
				circuitConnections[minIndex] = append(circuitConnections[minIndex], b)
			}
			circuitConnections[maxIndex] = nil
			continue
		}

		if existsOne {
			// fmt.Printf("b1 exists, %d, %v, %v\n", indexOne, pairs.box1, pairs.box2)
			circuits[pairs.box2] = indexOne
			circuitConnections[indexOne] = append(circuitConnections[indexOne], pairs.box2)
		} else if existsTwo {
			// fmt.Printf("b2 exists, %d, %v, %v\n", indexTwo, pairs.box1, pairs.box2)
			circuits[pairs.box1] = indexTwo
			circuitConnections[indexTwo] = append(circuitConnections[indexTwo], pairs.box1)
		} else {
			// fmt.Printf("b circuit %d, %v, %v\n", circuitIndex, pairs.box1, pairs.box2)
			circuits[pairs.box1] = circuitIndex
			circuits[pairs.box2] = circuitIndex
			circuitConnections[circuitIndex] = []Box{pairs.box1, pairs.box2}
			circuitIndex++
		}

	}

	// Extract keys into a slice
	connectionKeys := make([]int, 0, len(circuitConnections))
	for k := range circuitConnections {
		connectionKeys = append(connectionKeys, k)
	}

	// Sort keys by their values
	sort.Slice(connectionKeys, func(i, j int) bool {
		return len(circuitConnections[connectionKeys[i]]) > len(circuitConnections[connectionKeys[j]])
	})
	// for k, v := range circuitConnections {
	// 	fmt.Printf("%+v, %d\n", k, v)
	// }

	top := connectionKeys[:3]

	result := 1

	for _, k := range top {
		result *= len(circuitConnections[k])
	}

	return result
}

func PartTwo() int {
	circuits := make(map[Box]int)
	circuitConnections := make(map[int][]Box)

	boxes, err := parser("src/day08/input.in")
	if err != nil {
		log.Fatal("Failed to parse")
	}

	distances := computeDistances(boxes)
	keys := make([]BoxPair, 0, len(distances))
	for k := range distances {
		keys = append(keys, k)
	}

	// Sort keys by their values
	sort.Slice(keys, func(i, j int) bool {
		return distances[keys[i]] < distances[keys[j]] // ascending order
	})

	circuitIndex := 1

	result := 0

	for len(boxes) != len(circuitConnections[1]) {

		pairs := keys[:1][0]
		keys = keys[1:]

		indexOne, existsOne := circuits[pairs.box1]
		indexTwo, existsTwo := circuits[pairs.box2]

		if existsOne && existsTwo && indexOne == indexTwo {
			// fmt.Printf("b1 b2 both exits same index, %d %v %v\n", indexOne, pairs.box1, pairs.box2)
			continue
		}

		if existsOne && existsTwo {
			minIndex := min(indexOne, indexTwo)
			maxIndex := max(indexOne, indexTwo)

			// fmt.Printf("b1 b2 both exits, %d %v %v\n", minIndex, pairs.box1, pairs.box2)
			for _, b := range circuitConnections[maxIndex] {
				circuits[b] = minIndex
				circuitConnections[minIndex] = append(circuitConnections[minIndex], b)
			}

			if len(boxes) == len(circuitConnections[1]) {
				result = pairs.box1.x * pairs.box2.x
			}

			circuitConnections[maxIndex] = nil
			continue
		}

		if existsOne {
			// fmt.Printf("b1 exists, %d, %v, %v\n", indexOne, pairs.box1, pairs.box2)
			circuits[pairs.box2] = indexOne
			circuitConnections[indexOne] = append(circuitConnections[indexOne], pairs.box2)
		} else if existsTwo {
			// fmt.Printf("b2 exists, %d, %v, %v\n", indexTwo, pairs.box1, pairs.box2)
			circuits[pairs.box1] = indexTwo
			circuitConnections[indexTwo] = append(circuitConnections[indexTwo], pairs.box1)
		} else {
			// fmt.Printf("b circuit %d, %v, %v\n", circuitIndex, pairs.box1, pairs.box2)
			circuits[pairs.box1] = circuitIndex
			circuits[pairs.box2] = circuitIndex
			circuitConnections[circuitIndex] = []Box{pairs.box1, pairs.box2}
			circuitIndex++
		}
		if len(boxes) == len(circuitConnections[1]) {
			result = pairs.box1.x * pairs.box2.x
		}

	}

	return result
}

func computeDistances(boxes []Box) map[BoxPair]float64 {
	distances := make(map[BoxPair]float64)

	for i := range boxes {
		box1 := boxes[i]
		for j := i + 1; j < len(boxes); j++ {
			box2 := boxes[j]

			distances[BoxPair{box1, box2}] = distance(box1, box2)
		}
	}
	return distances
}

func distance(boxA, boxB Box) float64 {
	dx := float64(boxB.x - boxA.x)
	dy := float64(boxB.y - boxA.y)
	dz := float64(boxB.z - boxA.z)
	return math.Sqrt(dx*dx + dy*dy + dz*dz)
}

func parser(filePath string) ([]Box, error) {
	boxes := []Box{}
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return boxes, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

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
		z, err := strconv.Atoi(parts[2])
		if err != nil {
			log.Fatalf("Failed to convert %s to int\n", parts[2])
		}
		box := Box{x: x, y: y, z: z}
		boxes = append(boxes, box)

	}

	return boxes, nil
}
