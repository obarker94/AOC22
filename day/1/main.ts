import { data } from "./data";

interface ElfCalories {
  index: number;
  calories: number;
}

let arraysOfElfCalories: number[][] = [[]];
let numberOfElfs = 0;

for (let i = 0; i < data.length; i++) {
  if (data[i] === 999999) {
    numberOfElfs++;
    arraysOfElfCalories.push([]);
  } else {
    arraysOfElfCalories[numberOfElfs].push(data[i]);
  }
}

let summedArraysOfElfCalories: ElfCalories[] = [];

for (let i = 0; i < arraysOfElfCalories.length; i++) {
  const summedElfCalorieObject = {
    index: i,
    calories: arraysOfElfCalories[i].reduce((a, b) => a + b, 0),
  };

  summedArraysOfElfCalories.push(summedElfCalorieObject);
}

const sortedElves = summedArraysOfElfCalories.sort((a, b) => {
  return b.calories - a.calories;
});

const topThreeElves = sortedElves.slice(0, 3);

const summedCalories = topThreeElves.reduce((a, b) => a + b.calories, 0);

console.log(summedCalories);
