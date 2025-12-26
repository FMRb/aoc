// Package day10 - advent of Code
package day10

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Machine struct {
	indicator uint16
	buttons   []uint16
	iButtons  [][]int
	joltages  []int
}

type StatusMachine struct {
	level  int
	status uint16
}

type StatusJotaltageMachine struct {
	status         []int
	pressesCounter int
}

type Queue[T any] struct {
	items []T
}

func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{
		items: make([]T, 0),
	}
}

func (q *Queue[T]) Enqueue(item T) {
	q.items = append(q.items, item)
}

func (q *Queue[T]) Dequeue() (T, bool) {
	if len(q.items) == 0 {
		var zero T
		return zero, false
	}

	item := q.items[0]
	q.items = q.items[1:]

	return item, true
}

func (q *Queue[T]) Peek() (T, bool) {
	if len(q.items) == 0 {
		var zero T
		return zero, false
	}

	return q.items[0], true
}

func (q *Queue[T]) isEmpty() bool {
	return len(q.items) == 0
}

func (q *Queue[T]) Size() int {
	return len(q.items)
}

func PartOne() int {
	machines, err := parser("src/day10/example.in")
	if err != nil {
		log.Fatalf("Parse failed %v", err)
	}

	total := 0
	for _, m := range machines {
		numPressButtons, err := computeMachine(m)
		if err != nil {
			log.Fatalf("Compute machine failed %v\n", err)
		}
		total += numPressButtons
	}

	return total
}

func PartTwo() int {
	machines, err := parser("src/day10/input.in")
	if err != nil {
		log.Fatalf("Parse failed %v", err)
	}

	total := 0
	for i, m := range machines {
		minNumButtonPress, err := computeMachineJoltage(m)
		if err != nil {
			log.Fatalf("Compute machine failed %v\n", err)
		}
		fmt.Printf("%d/%d\n", i, len(machines))
		total += minNumButtonPress
	}

	return total
}

func computeMachineJoltage(m Machine) (int, error) {
	options := m.iButtons
	queue := NewQueue[StatusJotaltageMachine]()

	initStatus := make([]int, len(m.joltages))
	queue.Enqueue(StatusJotaltageMachine{initStatus, 0})

	for {
		item, exist := queue.Dequeue()
		if !exist {
			return 0, fmt.Errorf("no status in queuue")
		}

		if !isValidStatus(item.status, m.joltages) {
			continue
		}

		if slices.Equal(item.status, m.joltages) {
			return item.pressesCounter, nil
		}

		for _, o := range options {
			newStatus := make([]int, len(item.status))
			copy(newStatus, item.status)
			for _, i := range o {
				newStatus[i] += 1
			}

			queue.Enqueue(StatusJotaltageMachine{newStatus, item.pressesCounter + 1})
		}

	}
}

func isValidStatus(status []int, goal []int) bool {
	for i, v := range status {
		if v > goal[i] {
			return false
		}
	}
	return true
}

func computeMachine(m Machine) (int, error) {
	options := m.buttons
	queue := NewQueue[StatusMachine]()
	queue.Enqueue(StatusMachine{level: 0, status: uint16(0)})

	maxLevels := 10

	for {
		item, exist := queue.Dequeue()
		if !exist {
			return 0, fmt.Errorf("no status in queue")
		}

		if item.level == maxLevels {
			break
		}

		for _, o := range options {
			result := item.status ^ o

			// if item.level <= 5 && idx == 173 {
			// 	fmt.Printf("lvl: %d, (%08b ^ %08b) %08b  == %08b\n", item.level, item.status, o, result, m.indicator)
			// }

			if result == m.indicator {
				return item.level + 1, nil
			}

			queue.Enqueue(StatusMachine{item.level + 1, result})
		}

	}

	return maxLevels, nil
}

func parser(filePath string) ([]Machine, error) {
	machines := []Machine{}
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return machines, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		parts := strings.Split(line, " ")
		contentA := parts[0]
		contentB := parts[1 : len(parts)-1]
		contentC := parts[len(parts)-1]

		contentA = contentA[1 : len(contentA)-1]

		var indicator uint16
		for i, c := range strings.Split(contentA, "") {
			if c == "#" {
				indicator |= (1 << i)
			}
		}

		buttons := []uint16{}
		iButtons := [][]int{}
		for _, b := range contentB {
			b = b[1 : len(b)-1]

			button := uint16(0)
			iButton := []int{}

			for bi := range strings.SplitSeq(b, ",") {
				i, err := strconv.Atoi(bi)
				if err != nil {
					return []Machine{}, err
				}

				iButton = append(iButton, i)
				button |= (1 << i)
			}
			iButtons = append(iButtons, iButton)
			buttons = append(buttons, button)
		}

		contentC = contentC[1 : len(contentC)-1]

		joltages := []int{}
		for f := range strings.SplitSeq(contentC, ",") {

			i, err := strconv.Atoi(f)
			if err != nil {
				return []Machine{}, err
			}
			joltages = append(joltages, i)

		}

		machines = append(machines, Machine{indicator, buttons, iButtons, joltages})
	}

	return machines, nil
}
