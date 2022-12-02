import { data } from "./data";

// 1 rock (A || X)
// 2 paper ( B || Y)
// 3 scisccors ( C || Z)

// 6 points for win + choice
// 3 points for draw + choice
// 0 points for loss + choice

const choiceMatrix = {
  A: 1,
  X: 1,
  B: 2,
  Y: 2,
  C: 3,
  Z: 3,
};

const parsedData = data.split("\n").slice(1);
const roundData = parsedData.map((round) => round.split(" "));
const roundDataWithoutNaN = roundData.slice(0, roundData.length - 1);

const roundDataValues = roundDataWithoutNaN.map((round) => {
  const [player1, player2] = round;
  const player1Value = choiceMatrix[player1];
  const player2Value = choiceMatrix[player2];

  if (player1Value === player2Value) return 3 + player2Value;
  else if (player1Value === 1 && player2Value === 2) return 6 + player2Value;
  else if (player1Value === 1 && player2Value === 3) return 0 + player2Value;
  else if (player1Value === 2 && player2Value === 1) return 0 + player2Value;
  else if (player1Value === 2 && player2Value === 3) return 6 + player2Value;
  else if (player1Value === 3 && player2Value === 1) return 6 + player2Value;
  else return 0 + player2Value;
});

const sumOfAllRoundDataValues = roundDataValues.reduce(
  (acc, curr) => acc + curr
);

console.log(sumOfAllRoundDataValues);
