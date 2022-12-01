import path from 'path';
import fs from 'fs';

const getInputPath = (year: number | string, day: number | string) =>
  path.join(__dirname, `../input/Y${year}/Day${day}.txt`);

export interface Solution<A, B> {
  part1: (input: string) => A;
  part2: (input: string) => B;
  answers: [A, B];
}

const verify = (
  year: string,
  day: string,
  sol: Solution<number, number>,
): boolean => {
  const input = fs.readFileSync(getInputPath(year, day)).toString();

  const part1 = sol.part1(input.slice());
  const part2 = sol.part2(input.slice());

  let correct = true;
  if (part1 != sol.answers[0]) {
    console.error(
      `Error on part 1: Y${year}/${day} :> Expected ${sol.answers[0]} but got ${part1}`,
    );
    correct = false;
  }
  if (part2 != sol.answers[1]) {
    console.error(
      `Error on part 2: Y${year}/${day} :> Expected ${sol.answers[1]} but got ${part2}`,
    );
    correct = false;
  }

  return correct;
};

const argv = process.argv.slice(2);
const options: Record<string, boolean> = {
  '--test-all': false,
};
const params: string[] = [];

for (let i = 0; i < argv.length; i++) {
  if (argv[i] in options) {
    options[argv[i]] = true;
  } else if (argv[i][0] != '-') {
    params.push(argv[i]);
  } else {
    console.error(`Unkown option: ${argv[i]}`);
    process.exit(1);
  }
}

if (options['--test-all']) {
  const dirs = fs.readdirSync(__dirname, { withFileTypes: true });
  for (const dir of dirs) {
    if (dir.isDirectory()) {
      const year = dir.name.match(/Y(\d+)/);

      if (!year || !year[1]) continue;

      let yearDone = true;
      const files = fs.readdirSync(path.join(__dirname, dir.name), {
        withFileTypes: true,
      });
      for (const file of files) {
        if (path.extname(file.name) == '.ts') {
          const filePath = path.join(__dirname, dir.name, file.name);
          const day = file.name.match(/Day(\d+)/);

          if (!day || !day[1]) continue;

          const solution: Solution<number, number> = require(filePath).default;
          const correct = verify(year[1], day[1], solution);
          yearDone &&= correct;
        }
      }

      if (yearDone) {
        console.log(`Year ${year}: all done!`);
      }
    }
  }
} else {
  if (params.length < 2) {
    console.error('Please provide year and day to test');
    process.exit(1);
  }

  let [year, day] = params;
  if (isNaN(+year) || isNaN(+day)) {
    console.error('Please provide year and day as numbers');
    process.exit(1);
  }

  const solution: Solution<number, number> = require(path.join(
    __dirname,
    `Y${year}/Day${day}.ts`,
  )).default;
  const correct = verify(year, day, solution);
  if (correct) {
    console.log('All done!');
  }
}
