//go:generate peg -inline -switch grammar.peg

package policyandpasswordparser

import (
	"strings"
)

type Policy struct {
	RangeMin  int
	RangeMax  int
	Character byte
}

type Password string

// Option function.
type Option func(*parser)

func Parse(s string, options ...Option) (Policy, Password, error) {
	p := &parser{
		Buffer: strings.ToLower(s),
	}

	for _, o := range options {
		o(p)
	}

	p.Init()

	if err := p.Parse(); err != nil {
		return Policy{}, Password(""), err
	}

	p.Execute()

	policy := Policy{
		RangeMin:  p.policyRangeMin,
		RangeMax:  p.policyRangeMax,
		Character: p.policyChar,
	}

	password := Password(p.password)

	return policy, password, nil
}
