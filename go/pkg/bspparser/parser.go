//go:generate peg -inline -switch grammar.peg

package bspparser

// Option function.
type Option func(*parser)

func Parse(s string, options ...Option) (Seat, error) {
	p := &parser{
		Buffer: s,
	}

	for _, o := range options {
		o(p)
	}

	p.Init()

	if err := p.Parse(); err != nil {
		return Seat{}, err
	}

	p.Execute()

	return p.seat, nil
}
