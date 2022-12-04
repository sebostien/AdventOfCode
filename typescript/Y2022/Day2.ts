import { Solution } from '../run';

const toScore = (c: string): number =>
  (['A', 'B', 'C', 'X', 'Y', 'Z'].indexOf(c) % 3) + 1 || -Infinity;

const part1 = (input: string) => {
  let score = 0;

  for (const line of input.trim().split('\n')) {
    const [a, x] = line.trim().split(' ').map(toScore);
    score += x;
    if (a == x) {
      score += 3;
    } else if ((x == 3 && a == 2) || (x == 2 && a == 1) || (x == 1 && a == 3)) {
      score += 6;
    }
  }

  return score;
};

const part2 = (input: string) => {
  let score = 0;

  for (const line of input.trim().split('\n')) {
    const [a, x] = line.trim().split(' ').map(toScore);
    score += (x - 1) * 3;
    if (x == 1) score += a - 1 || 3;
    else if (x == 2) score += a;
    else if (x == 3) score += (a % 3) + 1;
  }

  return score;
};

const solution: Solution<number, number> = {
  part1,
  part2,
  answers: [8392, 10116],
};

export default solution;
