//go:generate peg -inline -switch grammar.peg

package passportparser

import (
	"strings"
)

// Option function.
type Option func(*parser)

func Parse(s string, options ...Option) (Passport, error) {
	p := &parser{
		Buffer: strings.ToLower(s),
	}

	for _, o := range options {
		o(p)
	}

	p.Init()

	if err := p.Parse(); err != nil {
		return Passport{}, err
	}

	p.Execute()

	return p.passport, nil
}

type Passport struct {
	byr int    // (Birth Year)
	iyr int    // (Issue Year)
	eyr int    // (Expiration Year)
	hgt length // (Height)
	hcl color  // (Hair Color)
	ecl color  // (Eye Color)
	pid string // (Passport ID)
	cid string // (Country ID) OPTIONAL
}

func (self Passport) IsValid() bool {
	return self.byr != 0 &&
		self.iyr != 0 &&
		self.eyr != 0 &&
		self.hgt.value != 0 &&
		self.hgt.units != dimensionUndefined &&
		self.hcl != "" &&
		self.ecl != "" &&
		self.pid != ""
}

type length struct {
	value int
	units dimension
}

type dimension string

const (
	dimensionUndefined dimension = ""
	dimensionCM        dimension = "cm"
	dimensionIN        dimension = "in"
)

type color string
