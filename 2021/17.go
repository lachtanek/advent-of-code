package main

import (
	"log"
	"math"
)

type Probe struct {
	pos  Point32
	xv   int
	yv   int
	step int
}

type ProbeTarget struct {
	minX int
	maxX int
	minY int
	maxY int
}

func launchProbe(xv, yv int) Probe {
	return Probe{Point32{0, 0}, xv, yv, 0}
}

func (probe *Probe) advance() {
	probe.pos.x += probe.xv
	probe.pos.y += probe.yv
	// probe.yv changing by 1 toward 0
	if probe.xv > 0 {
		probe.xv -= 1
	} else if probe.xv < 0 {
		probe.xv += 1
	}
	probe.yv -= 1
	probe.step += 1
}

func (probe Probe) hit(target ProbeTarget) bool {
	return probe.pos.x >= target.minX && probe.pos.y >= target.minY && probe.pos.x <= target.maxX && probe.pos.y <= target.maxY
}

func (probe Probe) overflew(target ProbeTarget) bool {
	return probe.pos.x > target.maxX || probe.pos.y < target.minY
}

func getMaxProbeYForLaunches(target ProbeTarget, minX, maxX, minY, maxY, testSteps int) int {
	bestY := math.MinInt32

	for x := minX; x < maxX; x++ {
		for y := minY; y < maxY; y++ {
			probe := launchProbe(x, y)
			bestProbeY := math.MinInt32
			hit := false

			for step := 0; step < testSteps; step++ {
				probe.advance()

				if probe.hit(target) {
					hit = true
					break
				}

				if probe.overflew(target) {
					break
				}

				if probe.pos.y > bestProbeY {
					bestProbeY = probe.pos.y
				}
			}

			if hit && bestProbeY > bestY {
				bestY = bestProbeY
			}
		}
	}

	return bestY
}

func getNumHitsForProbeLaunches(target ProbeTarget, minX, maxX, minY, maxY, testSteps int) int {
	hits := 0

	for x := minX; x < maxX; x++ {
		for y := minY; y < maxY; y++ {
			probe := launchProbe(x, y)

			for step := 0; step < testSteps; step++ {
				probe.advance()

				if probe.hit(target) {
					hits += 1
					break
				}

				if probe.overflew(target) {
					break
				}
			}
		}
	}

	return hits
}

func main17() {
	// testTarget := ProbeTarget{20, 30, -10, -5}
	// log.Println(getMaxProbeYForLaunches(testTarget, 0, 30, 0, 30, 20))

	target := ProbeTarget{102, 157, -146, -90}
	log.Println(getMaxProbeYForLaunches(target, -500, 500, -500, 500, 2000))
	log.Println(getNumHitsForProbeLaunches(target, -500, 500, -500, 500, 2000))
}
