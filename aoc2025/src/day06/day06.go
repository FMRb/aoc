// Package day06 - advent of Code
package day06

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func PartOne() int {
	problems, err := parser("src/day06/input.in")
	if err != nil {
		log.Fatal("Parser failed ", err)
	}

	operators := []rune{}
	operandss := [][]int{}
	for i, p := range problems {
		if i == len(problems)-1 {
			operators = []rune(strings.Join(p, ""))
		} else {
			operands := []int{}
			for _, o := range p {
				n, err := strconv.Atoi(o)
				if err != nil {
					log.Fatalf("Error Atoi %s, %v", o, err)
				}
				operands = append(operands, n)
			}
			operandss = append(operandss, operands)
		}
	}

	sortedOperandss := [][]int{}

	for i := 0; i < len(operandss[0]); i++ {
		sortedOperands := []int{}

		for _, operands := range operandss {
			sortedOperands = append(sortedOperands, operands[i])
		}

		sortedOperandss = append(sortedOperandss, sortedOperands)
	}

	result := 0

	for i, operands := range sortedOperandss {

		temp := 0

		if operators[i] == '*' {
			temp = 1
		}

		for _, operand := range operands {
			if operators[i] == '*' {
				temp *= operand
			} else {
				temp += operand
			}
		}

		result += temp
	}
	return result
}

func PartTwo() int {
	lines, err := parserTwo("src/day06/input.in")
	if err != nil {
		log.Fatal("Parser failed ", err)
	}

	rlines := []string{}

	for _, l := range lines {
		for j, s := range l {
			if j < len(rlines) {
				rlines[j] = rlines[j] + string(s)
			} else {
				rlines = append(rlines, string(s))
			}
		}
	}

	operator := '+'
	total := 0

	temp := 0
	for _, s := range rlines {
		if len(strings.TrimSpace(s)) == 0 {
			total += temp
			continue
		}
		if strings.ContainsRune(s, '*') {
			operator = rune(s[len(s)-1])
			operand := strings.TrimSpace(s[:len(s)-1])

			t, err := strconv.Atoi(operand)
			if err != nil {
				log.Fatalf("Failed convert number: %s %v\n", s[:len(s)-1], err)
			}
			temp = t
			continue
		} else if strings.ContainsRune(s, '+') {
			operator = rune(s[len(s)-1])
			operand := strings.TrimSpace(s[:len(s)-1])
			t, err := strconv.Atoi(operand)
			if err != nil {
				log.Fatalf("Failed convert number: %s %v\n", s[:len(s)-1], err)
			}
			temp = t
			continue
		}

		if operator == '+' {
			s = strings.TrimSpace(s)
			t, err := strconv.Atoi(s)
			if err != nil {
				log.Fatalf("Failed convert number: %s %v\n", s, err)
			}
			temp += t
		}

		if operator == '*' {
			s = strings.TrimSpace(s)
			t, err := strconv.Atoi(s)
			if err != nil {
				log.Fatalf("Failed convert number: %s %v\n", s, err)
			}
			temp *= t
		}

	}
	total += temp
	return total
}

func parser(filePath string) ([][]string, error) {
	problems := [][]string{}
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return [][]string{}, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		problems = append(problems, strings.Fields(line))
	}

	return problems, nil
}

func parserTwo(filePath string) ([]string, error) {
	problems := []string{}
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Failed to read file %s\n", filePath)
		return []string{}, err
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		problems = append(problems, line)
	}

	return problems, nil
}
