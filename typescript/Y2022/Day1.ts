import { Solution } from '../run';

const part1 = (input: string) => {
  const rows: string[] = input.trim().split('\n\n');

  return Math.max(
    ...rows.map((row) => row.split('\n').reduce((acc, val) => acc + +val, 0)),
  );
};

const part2 = (input: string) => {
  const rows: string[] = input.trim().split('\n\n');

  const [a, b, c] = rows
    .map((row) => row.split('\n').reduce((acc, val) => acc + +val, 0))
    .sort((a, b) => b - a);

  return a + b + c;
};

const solution: Solution<number, number> = {
  part1,
  part2,
  answers: [65912, 195625],
};

export default solution;
