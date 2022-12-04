import { Solution } from '../run';

type Range = [number, number];

const parseRow = (row: string): [Range, Range] => {
  return row.split(',').map((x) => x.split('-').map((y) => +y)) as [
    Range,
    Range,
  ];
};

const contained = (a: Range, b: Range): boolean => a[0] <= b[0] && a[1] >= b[1];

const part1 = (input: string) => {
  return input
    .trim()
    .split('\n')
    .map(parseRow)
    .reduce((acc, [a, b]) => acc + +(contained(a, b) || contained(b, a)), 0);
};

const intersects = (a: Range, b: Range): boolean =>
  a[1] >= b[0] && a[0] <= b[1];

const part2 = (input: string) => {
  return input
    .trim()
    .split('\n')
    .map(parseRow)
    .reduce((acc, [a, b]) => acc + +intersects(a, b), 0);
};

const solution: Solution<number, number> = {
  part1,
  part2,
  answers: [526, 886],
};

export default solution;
