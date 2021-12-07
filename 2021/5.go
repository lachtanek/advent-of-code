package main

import (
	"log"
	"math"
	"strconv"
	"strings"
)

type Point struct {
	x int64
	y int64
}

type Line struct {
	p1 Point
	p2 Point
}

type Field = [][]int64

const MAX_DIST = 1000

func parsePoint(data string) Point {
	splits := strings.Split(data, ",")
	x, _ := strconv.ParseInt(splits[0], 10, 64)
	y, _ := strconv.ParseInt(splits[1], 10, 64)
	return Point{x, y}
}

func parseLines(data []string) []Line {
	lines := []Line{}

	for _, line := range data {
		points := strings.Split(line, " -> ")
		p1 := parsePoint(points[0])
		p2 := parsePoint(points[1])
		lines = append(lines, Line{p1, p2})
	}

	return lines
}

func isHorizontal(line Line) bool {
	return line.p1.y == line.p2.y
}

func isVertical(line Line) bool {
	return line.p1.x == line.p2.x
}

func getStartEnd(c1, c2 int64) [2]int64 {
	if c1 <= c2 {
		return [2]int64{c1, c2}
	} else {
		return [2]int64{c2, c1}
	}
}

func getSign(c1, c2 int64) int64 {
	if c1 <= c2 {
		return 1
	} else {
		return -1
	}
}

func xyCond(val, c1, c2 int64) bool {
	if c1 <= c2 {
		return val <= c2
	} else {
		return val >= c2
	}
}

func addAllPointsFromLine(field *Field, line Line) {
	xSign := getSign(line.p1.x, line.p2.x)
	ySign := getSign(line.p1.y, line.p2.y)
	var x, y int64

	if isHorizontal(line) || isVertical(line) {
		for y = line.p1.y; xyCond(y, line.p1.y, line.p2.y); y += ySign {
			for x = line.p1.x; xyCond(x, line.p1.x, line.p2.x); x += xSign {
				(*field)[y][x] += 1
			}
		}
	} else {
		dist := int64(math.Abs(float64(line.p1.x - line.p2.x)))
		var c int64

		if ySign == xSign {
			for c = 0; c <= dist; c++ {
				(*field)[line.p1.y+(c*ySign)][line.p1.x+(c*xSign)] += 1
			}
		} else {
			if ySign == -1 {
				line = Line{line.p2, line.p1}
			}

			for c = 0; c <= dist; c++ {
				(*field)[line.p1.y+c][line.p1.x-c] += 1
			}
		}
	}
}

func getFieldWithStraightLines(lines []Line) Field {
	field := make(Field, MAX_DIST)

	for y := 0; y < MAX_DIST; y++ {
		field[y] = make([]int64, MAX_DIST)
	}

	for _, line := range lines {
		if isHorizontal(line) {
			startEnd := getStartEnd(line.p1.x, line.p2.x)
			for x := startEnd[0]; x <= startEnd[1]; x++ {
				field[line.p1.y][x] += 1
			}
		} else if isVertical(line) {
			startEnd := getStartEnd(line.p1.y, line.p2.y)
			for y := startEnd[0]; y <= startEnd[1]; y++ {
				field[y][line.p1.x] += 1
			}
		}
	}

	return field
}

func getFieldWithDiagonalLines(lines []Line) Field {
	field := make(Field, MAX_DIST)

	for y := 0; y < MAX_DIST; y++ {
		field[y] = make([]int64, MAX_DIST)
	}

	for _, line := range lines {
		addAllPointsFromLine(&field, line)
	}

	return field
}

func countIntersects(field Field) int {
	count := 0

	for x := 0; x < MAX_DIST; x++ {
		for y := 0; y < MAX_DIST; y++ {
			if field[y][x] > 1 {
				count++
			}
		}
	}

	return count
}

func main5() {
	data, err := ReadInputFrom("5.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	lines := parseLines(data)

	log.Println(countIntersects(getFieldWithStraightLines(lines)))
	log.Println(countIntersects(getFieldWithDiagonalLines(lines)))
}
