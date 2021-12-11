package main

import (
	"log"
)

func parseFlashMap(data []string) HeightMap {
	heightMap := HeightMap{[][]int{}}

	for _, line := range data {
		energies := []int{}
		for _, energy := range line {
			energies = append(energies, int(energy-'0'))
		}

		heightMap.points = append(heightMap.points, energies)
	}

	return heightMap
}

func (heightMap HeightMap) contains(point Point32) bool {
	return point.x >= 0 && point.y >= 0 && point.x < heightMap.xsize() && point.y < heightMap.ysize()
}

func (heightMap HeightMap) getAllAdjacentPoints(point Point32) []Point32 {
	adjacent := []Point32{}

	for y := -1; y <= 1; y++ {
		for x := -1; x <= 1; x++ {
			point := Point32{point.x + x, point.y + y}
			if heightMap.contains(point) {
				adjacent = append(adjacent, point)
			}
		}
	}

	return adjacent
}

func flash(heightMap HeightMap, point Point32) int {
	flashes := 1

	for _, adj := range heightMap.getAllAdjacentPoints(point) {
		heightMap.points[adj.y][adj.x] += 1

		if heightMap.points[adj.y][adj.x] == 10 {
			flashes += flash(heightMap, adj)
		}
	}

	return flashes
}

func countFlashes(heightMap HeightMap, days int) int {
	totalFlashes := 0

	for i := 0; i < days; i++ {
		for y := 0; y < heightMap.ysize(); y++ {
			for x := 0; x < heightMap.xsize(); x++ {
				heightMap.points[y][x] += 1

				if heightMap.points[y][x] == 10 {
					totalFlashes += flash(heightMap, Point32{x, y})
				}
			}
		}

		for y := 0; y < heightMap.ysize(); y++ {
			for x := 0; x < heightMap.xsize(); x++ {
				if heightMap.points[y][x] >= 10 {
					heightMap.points[y][x] = 0
				}
			}
		}
	}

	return totalFlashes
}

func allFlashAt(heightMap HeightMap) int {
	for i := 0; ; i++ {
		stepFlashes := 0

		for y := 0; y < heightMap.ysize(); y++ {
			for x := 0; x < heightMap.xsize(); x++ {
				heightMap.points[y][x] += 1

				if heightMap.points[y][x] == 10 {
					stepFlashes += flash(heightMap, Point32{x, y})
				}
			}
		}

		for y := 0; y < heightMap.ysize(); y++ {
			for x := 0; x < heightMap.xsize(); x++ {
				if heightMap.points[y][x] >= 10 {
					heightMap.points[y][x] = 0
				}
			}
		}

		if stepFlashes == heightMap.ysize()*heightMap.xsize() {
			return i + 1
		}
	}
}

func main11() {
	data, err := ReadInputFrom("11.inp")

	if err != nil {
		log.Fatal(err)
		return
	}

	flashMap := parseFlashMap(data)

	log.Println(countFlashes(flashMap, 100))

	flashMap2 := parseFlashMap(data)
	log.Println(allFlashAt(flashMap2))
}
