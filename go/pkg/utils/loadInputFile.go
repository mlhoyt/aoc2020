package utils

import (
	"bufio"
	"os"
	"path/filepath"
)

func LoadInputFile(name string) ([]string, error) {
	absName, err := filepath.Abs("input/" + name)
	if err != nil {
		return nil, err
	}

	ifh, err := os.Open(absName)
	if err != nil {
		return nil, err
	}
	defer ifh.Close()

	lines := []string{}
	scanner := bufio.NewScanner(ifh)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}
