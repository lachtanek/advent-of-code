package main

import (
	"bufio"
	"os"
	"strconv"
)

func ReadNumbersInputFrom(filename string) ([]int, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	defer file.Close()

	numbers := []int{}
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		number, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return nil, err
		}

		numbers = append(numbers, number)
	}

	err = scanner.Err()
	if err != nil {
		return nil, err
	}

	return numbers, nil
}

func ReadInputFrom(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	defer file.Close()

	data := []string{}
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		data = append(data, line)
	}

	err = scanner.Err()
	if err != nil {
		return nil, err
	}

	return data, nil
}
