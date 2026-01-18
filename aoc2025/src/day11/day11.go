// Package day11 - advent of Code
package day11

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Device struct {
	name    string
	outputs []*Device
}

func PartOne() int {
	devicesMap, err := parser("src/day11/example.in")
	if err != nil {
		log.Fatalf("erro parser %v\n", err)
	}

	visited := make(map[string]bool)
	stack := []string{"you"}
	outCounter := 0
	for len(stack) > 0 {

		deviceName := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		visited[deviceName] = true

		devices, exist := devicesMap[deviceName]
		if !exist {
			log.Fatalf("device name do not exist, %s\n", deviceName)
		}

		for _, d := range devices {
			if d == "out" {
				outCounter += 1
			} else {
				stack = append(stack, d)
			}
		}

	}

	return outCounter
}

type Item struct {
	name  string
	level int
}

type CacheValue struct {
	value       int
	hasFFTOrDAC int
}

func PartTwo() int {
	devicesMap, err := parser("src/day11/example_two.in")
	if err != nil {
		log.Fatalf("erro parser %v\n", err)
	}

	startItem := Item{"svr", 1}

	cache := make(map[Item]CacheValue)
	outCounter := runNode(startItem, 0, devicesMap, cache)

	// stack := []Item{{"svr", 1}}
	// visited := make(map[Item]bool)
	// connection := []string{}
	// outCounter := 0
	// for len(stack) > 0 {

	// 	item := stack[len(stack)-1]
	// 	stack = stack[:len(stack)-1]

	// 	// if _, exists := visited[item]; exists {
	// 	// 	continue
	// 	// }

	// 	// visited[item] = true
	// 	if len(connection) > item.level {
	// 		connection = connection[:item.level-1]
	// 	}
	// 	connection = append(connection, item.name)

	// 	devices, exist := devicesMap[item.name]
	// 	if !exist {
	// 		log.Fatalf("device name do not exist, %s\n", item.name)
	// 	}

	// 	for _, d := range devices {
	// 		if d == "out" {
	// 			// fmt.Printf("%+v\n", connection)
	// 			if slices.Contains(connection, "fft") && slices.Contains(connection, "dac") {
	// 				fmt.Printf("%+v\n", connection)
	// 				outCounter += 1
	// 			}
	// 		} else {
	// 			newItem := Item{d, item.level + 1}
	// 			stack = append(stack, newItem)
	// 		}
	// 	}

	// }

	return outCounter
}

func runNode(item Item, hasFftAndDac int, devicesMap map[string][]string, cache map[Item]CacheValue) int {
	fmt.Printf("Node %s\n", item.name)
	if v, exists := cache[item]; exists {
		return v.value
	}

	if item.name == "out" {
		fmt.Printf("Oute %d\n", hasFftAndDac)
		if hasFftAndDac >= 2 {
			fmt.Printf("DAC & FFT %d\n", hasFftAndDac)
			return 1
		} else {
			return 0
		}
	}

	devices, exist := devicesMap[item.name]
	if !exist {
		log.Fatalf("device name do not exist, %s\n", item.name)
	}

	counter := 0

	for _, c := range devices {
		newItem := Item{name: c, level: item.level + 1}
		if c == "dac" || c == "fft" {
			fmt.Printf("Detecting %s - c %d\n", c, hasFftAndDac)
			hasFftAndDac += 1
		}
		counter += runNode(newItem, hasFftAndDac, devicesMap, cache)
	}
	cache[item] = counter

	return counter
}

func parser(filePath string) (map[string][]string, error) {
	devicesMap := make(map[string][]string)
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return devicesMap, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		parts := strings.Split(line, ":")

		deviceName := parts[0]

		devices := strings.Split(parts[1][1:], " ")
		devicesMap[deviceName] = devices
	}

	return devicesMap, nil
}
