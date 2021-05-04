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

	result := traverse(topo, 3, 1)

	fmt.Printf("%d\n", result)
}

func newTopoMap(s string) (parser.TopoMap, error) {
	return parser.Parse(s)
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
