import { readFileSync as fs_readFileSync } from 'fs';

// const FILE_NAME = "input/day01.test.txt";
const FILE_NAME = "input/day01.txt";

function main(): void {
  const input = fs_readFileSync(FILE_NAME, 'utf-8');
  // console.log("input: ", input);

  let numbers = input
    .split("\n")
    .map((s) => {return parseInt(s);});
  // console.log("numbers: ", numbers);

  let groups = generate_groups(numbers);
  // console.log("groups: ", groups);

  let answers = groups
    .filter(([x, y]) => { return (x + y) == 2020 });
  // console.log("answers: ", answers);

  answers.forEach(([x, y]) => { console.log(`${x} * ${y} = ${x * y}`); });
}

function generate_groups(xs: number[]): [number, number][] {
    let groups = [];

    xs.forEach((x, xi) => {
        let ys = xs.slice(xi+1,);
        ys.forEach((y) => {
            groups.push([x, y]);
        });
    })

    return groups
}

main()
