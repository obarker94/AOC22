import * as fs from "node:fs";

const input = fs.readFileSync("data.txt", "utf8");
const lines = String(input.split("\n"));
const parsedLines = lines.split(",").slice(0, -1);

const alpha = Array.from(Array(26)).map((_, i) => i + 65);
const alphabetLowercase = alpha.map((x) =>
  String.fromCharCode(x).toLowerCase()
);
const alphabetUppercase = alpha.map((x) => String.fromCharCode(x));
const alphabet = alphabetLowercase.concat(alphabetUppercase);

const alphabetMap = new Map();
for (let i = 0; i < alphabet.length; i++) {
  alphabetMap.set(alphabet[i], i + 1);
}

// part 1
const compartmentsForEachBackpack = parsedLines.map((line) => {
  const lengthOfLine = line.length;
  const middleOfLine = Math.floor(lengthOfLine / 2);
  const firstHalf = line.slice(0, middleOfLine);
  const secondHalf = line.slice(middleOfLine, lengthOfLine);
  return [firstHalf, secondHalf];
});

const duplicatesInEachBackpack = compartmentsForEachBackpack.map(
  (compartment) => {
    const [firstHalf, secondHalf] = compartment;

    const firstHalfCharacters = firstHalf.split("");
    const secondHalfCharacters = secondHalf.split("");

    const duplicates = firstHalfCharacters.filter((character) =>
      secondHalfCharacters.includes(character)
    );

    return duplicates.slice(0, 1);
  }
);

const duplicates = duplicatesInEachBackpack.flat();
const sumOfDuplicates = duplicates.reduce((acc, curr) => {
  const value = alphabetMap.get(curr);
  return acc + value;
}, 0);

console.log("priorty of duplicates in compartments : ", sumOfDuplicates);

// part 2

function* chunks<T>(arr: T[], n: number): Generator<T[], void> {
  for (let i = 0; i < arr.length; i += n) {
    yield arr.slice(i, i + n);
  }
}

const groupOfThreeBackpacks = Array.from(chunks(parsedLines, 3));

let commonItemsInEachGroup = groupOfThreeBackpacks.map((group) => {
  const [firstBackpack, secondBackpack, thirdBackpack] = group;

  const firstBackpackCharacters = firstBackpack.split("");
  const secondBackpackCharacters = secondBackpack.split("");
  const thirdBackpackCharacters = thirdBackpack.split("");

  const commonItems = firstBackpackCharacters.filter((character) =>
    secondBackpackCharacters.includes(character)
  );

  const commonItemsInGroup = commonItems.filter((character) =>
    thirdBackpackCharacters.includes(character)
  );

  return commonItemsInGroup.slice(0, 1);
});

const commonItems = commonItemsInEachGroup.flat();

const sumOfCommonItems = commonItems.reduce((acc, curr) => {
  const value = alphabetMap.get(curr);
  return acc + value;
}, 0);

console.log(sumOfCommonItems);
