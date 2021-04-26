package main

import (
	"fmt"
	"strings"

	parser "github.com/mlhoyt/aoc2020/go/pkg/policyandpasswordparser"
	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day02.txt")
	if err != nil {
		panic(err)
	}

	items, err := stringListToPasswordAndPolicyList(strings.Split(input, "\n"))
	if err != nil {
		panic(err)
	}

	items = filterPolicyAndPasswordList(items, func(x policyAndPassword) bool {
		count := strings.Count(string(x.password), string(x.policy.Character))
		return count >= x.policy.RangeMin && count <= x.policy.RangeMax
	})

	fmt.Printf("%d\n", len(items))
}

type policyAndPassword struct {
	policy   parser.Policy
	password parser.Password
}

type policyAndPasswordList []policyAndPassword

func stringListToPasswordAndPolicyList(ss []string) (policyAndPasswordList, error) {
	items := policyAndPasswordList{}
	for _, s := range ss {
		policy, password, err := parser.Parse(s)
		if err != nil {
			return nil, err
		}

		items = append(items, policyAndPassword{policy, password})
	}

	return items, nil
}

func filterPolicyAndPasswordList(xs policyAndPasswordList, f func(policyAndPassword) bool) policyAndPasswordList {
	results := policyAndPasswordList{}
	for _, x := range xs {
		if f(x) {
			results = append(results, x)
		}
	}

	return results
}
