import { Solution } from '../run';

const priority = (c: string): number => {
  if (c.toUpperCase() == c) {
    return c.charCodeAt(0) - 38;
  } else {
    return c.charCodeAt(0) - 96;
  }
};

const part1 = (input: string) => {
  let sum = 0;

  for (const line of input.trim().split('\n')) {
    const chars = line.trim().split('');
    const right = chars.slice(chars.length / 2);
    for (const c of chars) {
      if (right.includes(c)) {
        sum += priority(c);
        break;
      }
    }
  }

  return sum;
};

const part2 = (input: string) => {
  let sum = 0;

  const sacks = input
    .trim()
    .split('\n')
    .map((s) => new Set(s.trim().split('')));

  for (let i = 0; i < sacks.length; i++) {
    if (i % 3 == 0) {
      for (const c of sacks[i].values()) {
        if (sacks[i + 1].has(c) && sacks[i + 2].has(c)) {
          sum += priority(c);
          break;
        }
      }
    }
  }

  return sum;
};

const solution: Solution<number, number> = {
  part1,
  part2,
  answers: [7878, 2760],
};

export default solution;
