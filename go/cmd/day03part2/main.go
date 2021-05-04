package main

import (
	"fmt"

	parser "github.com/mlhoyt/aoc2020/go/pkg/topomapparser"

	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day03.txt")
	if err != nil {
		panic(err)
	}

	topo, err := newTopoMap(input)
	if err != nil {
		panic(err)
	}

	slopes := slopeList{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	}

	result := slopes.mapToIntList(func(v slope) int {
		return traverse(topo, v.x, v.y)
	}).product()

	fmt.Printf("%d\n", result)
}

func newTopoMap(s string) (parser.TopoMap, error) {
	return parser.Parse(s)
}

type slope struct {
	x int
	y int
}

type slopeList []slope

func (self slopeList) mapToIntList(f func(slope) int) intList {
	result := intList{}
	for _, slope := range self {
		result = append(result, f(slope))
	}
	return result
}

type intList []int

func (self intList) product() int {
	accum := 1
	for _, v := range self {
		accum *= v
	}
	return accum
}

type position struct {
	x int
	y int
}

func newPosition() position {
	return position{}
}

func (self position) isOnMap(m parser.TopoMap) bool {
	return self.y < m.GetHeight()
}

func (self *position) move(mvRight int, mvDown int) {
	self.x += mvRight
	self.y += mvDown
}

func (self position) isHit(m parser.TopoMap) bool {
	return m.GetCoordinate(self.x, self.y) == parser.TopoTree
}

func traverse(m parser.TopoMap, mvRight int, mvDown int) int {
	hits := 0
	for pos := newPosition(); pos.isOnMap(m); pos.move(mvRight, mvDown) {
		if pos.isHit(m) {
			hits++
		}
	}

	return hits
}
