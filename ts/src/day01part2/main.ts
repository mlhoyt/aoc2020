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
    .filter(([x, y, z]) => { return (x + y + z) == 2020 });
  // console.log("answers: ", answers);

  answers.forEach(([x, y, z]) => { console.log(`${x} * ${y} * ${z}= ${x * y * z}`); });
}

function generate_groups(xs: number[]): [number, number, number][] {
    let groups = [];

    xs.forEach((x, xi) => {
        let ys = xs.slice(xi+1,);
        ys.forEach((y, yi) => {
            let zs = ys.slice(yi+1,);
            zs.forEach((z) => {
                groups.push([x, y, z]);
            });
        });
    })

    return groups
}

main()
