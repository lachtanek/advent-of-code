package main

import (
	"log"
	"sort"
)

type HeightMap struct {
	points [][]int
}

type Point32 struct {
	x int
	y int
}

func parseHeightMap(data []string) HeightMap {
	heightMap := HeightMap{[][]int{}}

	for _, line := range data {
		heights := []int{}
		for _, height := range line {
			heights = append(heights, int(height-'0'))
		}

		heightMap.points = append(heightMap.points, heights)
	}

	return heightMap
}

func (heightMap HeightMap) get(point Point32) int {
	return heightMap.points[point.y][point.x]
}

func (heightMap HeightMap) xsize() int {
	return len(heightMap.points[0])
}

func (heightMap HeightMap) ysize() int {
	return len(heightMap.points)
}

func findLowPoints(heightMap HeightMap) []Point32 {
	points := []Point32{}

	for y := 0; y < heightMap.ysize(); y++ {
		for x := 0; x < heightMap.xsize(); x++ {
			point := Point32{x, y}
			value := heightMap.get(point)
			isLowest := true

			for _, adj := range heightMap.getAdjacentPoints(point) {
				if value >= heightMap.get(adj) {
					isLowest = false
					break
				}
			}

			if isLowest {
				points = append(points, Point32{x, y})
			}
		}
	}

	return points
}

func calculateRisk(heightMap HeightMap, lowPoints []Point32) int {
	risk := 0
	for _, point := range lowPoints {
		risk += heightMap.get(point) + 1
	}
	return risk
}

func contains(list []Point32, p Point32) bool {
	for _, e := range list {
		if p == e {
			return true
		}
	}

	return false
}

func (heightMap HeightMap) getAdjacentPoints(point Point32) []Point32 {
	adjacent := []Point32{}

	if point.x > 0 {
		adjacent = append(adjacent, Point32{point.x - 1, point.y})
	}
	if point.x < heightMap.xsize()-1 {
		adjacent = append(adjacent, Point32{point.x + 1, point.y})
	}
	if point.y > 0 {
		adjacent = append(adjacent, Point32{point.x, point.y - 1})
	}
	if point.y < heightMap.ysize()-1 {
		adjacent = append(adjacent, Point32{point.x, point.y + 1})
	}

	return adjacent
}

func findBasinPoints(heightMap HeightMap, lowPoint Point32) []Point32 {
	toVisit := []Point32{lowPoint}
	basinPoints := []Point32{lowPoint}

	for len(toVisit) > 0 {
		point := toVisit[0]
		value := heightMap.get(point)
		toVisit = toVisit[1:]

		numLower := 0
		for _, adj := range heightMap.getAdjacentPoints(point) {
			if contains(basinPoints, adj) {
				// ...
			} else if heightMap.get(adj) < value {
				numLower++
			} else if heightMap.get(adj) < 9 {
				toVisit = append(toVisit, adj)
			}
		}

		if numLower <= 1 {
			if !contains(basinPoints, point) {
				basinPoints = append(basinPoints, point)
			}
		}
	}

	return basinPoints
}

func find3LargestBasins(heightMap HeightMap, lowPoints []Point32) [][]Point32 {
	basins := [][]Point32{}
	sizes := []int{}

	for _, lowPoint := range lowPoints {
		basin := findBasinPoints(heightMap, lowPoint)
		basins = append(basins, basin)
		sizes = append(sizes, len(basin))
	}

	sort.SliceStable(basins, func(i, j int) bool {
		return len(basins[i]) > len(basins[j])
	})

	return basins[:3]
}

func basinSizeScore(basins [][]Point32) int {
	score := 1
	for _, basin := range basins {
		score *= len(basin)
	}

	return score
}

func main9() {
	data, err := ReadInputFrom("9.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	heightMap := parseHeightMap(data)
	lowPoints := findLowPoints(heightMap)

	log.Println(calculateRisk(heightMap, lowPoints))
	log.Println(basinSizeScore(find3LargestBasins(heightMap, lowPoints)))
}
