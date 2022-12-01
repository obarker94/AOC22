"use strict";
exports.__esModule = true;
var data_1 = require("./data");
var arraysOfElfCalories = [[]];
var numberOfElfs = 0;
for (var i = 0; i < data_1.data.length; i++) {
    if (data_1.data[i] === 999999) {
        numberOfElfs++;
        arraysOfElfCalories.push([]);
    }
    else {
        arraysOfElfCalories[numberOfElfs].push(data_1.data[i]);
    }
}
var summedArraysOfElfCalories = [];
for (var i = 0; i < arraysOfElfCalories.length; i++) {
    var summedElfCalorieObject = {
        index: i,
        calories: arraysOfElfCalories[i].reduce(function (a, b) { return a + b; }, 0)
    };
    summedArraysOfElfCalories.push(summedElfCalorieObject);
}
var sortedElves = summedArraysOfElfCalories.sort(function (a, b) {
    return b.calories - a.calories;
});
var topThreeElves = sortedElves.slice(0, 3);
var summedCalories = topThreeElves.reduce(function (a, b) { return a + b.calories; }, 0);
console.log(summedCalories);
