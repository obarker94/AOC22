"use strict";
exports.__esModule = true;
var data_1 = require("./data");
// 1 rock (A || X)
// 2 paper ( B || Y)
// 3 scisccors ( C || Z)
// 6 points for win + choice
// 3 points for draw + choice
// 0 points for loss + choice
var choiceMatrix = {
    A: 1,
    X: 1,
    B: 2,
    Y: 2,
    C: 3,
    Z: 3
};
var parsedData = data_1.data.split("\n").slice(1);
var roundData = parsedData.map(function (round) { return round.split(" "); });
var roundDataWithoutNaN = roundData.slice(0, roundData.length - 1);
var roundDataValues = roundDataWithoutNaN.map(function (round) {
    var player1 = round[0], player2 = round[1];
    switch (player2) {
        case "X":
            if (player1 === "A")
                return 3;
            else if (player1 === "B")
                return 1;
            else
                return 2;
        case "Y":
            return choiceMatrix[player1] + 3;
        case "Z":
            if (player1 === "A")
                return 2 + 6;
            else if (player1 === "B")
                return 3 + 6;
            else
                return 1 + 6;
        default:
            return 0;
    }
});
console.log(roundDataValues.slice(0, 5));
console.log(roundDataWithoutNaN.slice(0, 5));
// if (player1Value === player2Value) return 3 + player2Value;
// else if (player1Value === 1 && player2Value === 2) return 6 + player2Value;
// else if (player1Value === 1 && player2Value === 3) return 0 + player2Value;
// else if (player1Value === 2 && player2Value === 1) return 0 + player2Value;
// else if (player1Value === 2 && player2Value === 3) return 6 + player2Value;
// else if (player1Value === 3 && player2Value === 1) return 6 + player2Value;
// else return 0 + player2Value;
var sumOfAllRoundDataValues = roundDataValues.reduce(function (acc, curr) { return acc + curr; });
console.log(sumOfAllRoundDataValues);
