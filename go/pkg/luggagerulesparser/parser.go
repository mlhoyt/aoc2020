//go:generate peg -inline -switch grammar.peg

package luggagerulesparser

import (
	"strings"

	"github.com/mlhoyt/aoc2020/go/pkg/types"
)

// Option function.
type Option func(*parser)

func Parse(s string, options ...Option) (types.LuggageBagSpecs, error) {
	p := &parser{
		Buffer: strings.ToLower(s),
		specs:  types.LuggageBagSpecs{},
	}

	for _, o := range options {
		o(p)
	}

	p.Init()

	if err := p.Parse(); err != nil {
		return p.specs, err
	}

	p.Execute()

	return p.specs, nil
}
