//go:generate peg -inline -switch grammar.peg

package topomapparser

import (
	"strings"
)

// Option function.
type Option func(*parser)

func Parse(s string, options ...Option) (TopoMap, error) {
	p := &parser{
		Buffer: strings.ToLower(s),
	}

	for _, o := range options {
		o(p)
	}

	p.Init()

	if err := p.Parse(); err != nil {
		return TopoMap{}, err
	}

	p.Execute()

	return p.topoMap, nil
}

type TopoItem byte

const (
	TopoTree  TopoItem = '#'
	TopoSpace TopoItem = '.'
)

type TopoMap [][]TopoItem

func (self TopoMap) GetHeight() int {
	return len(self)
}

func (self TopoMap) GetWidth() int {
	if len(self) > 0 {
		return len(self[0])
	}

	return 0
}

func (self TopoMap) GetCoordinate(x int, y int) TopoItem {
	if y >= self.GetHeight() {
		return TopoSpace
	}

	if x >= self.GetWidth() {
		x = x % self.GetWidth()
	}

	return self[y][x]
}
