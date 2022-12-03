"use strict";
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
var fs = require("node:fs");
var hrstart = process.hrtime();
var input = fs.readFileSync("data.txt", "utf8");
var lines = String(input.split("\n"));
var parsedLines = lines.split(",").slice(0, -1);
var alpha = Array.from(Array(26)).map(function (_, i) { return i + 65; });
var alphabetLowercase = alpha.map(function (x) {
    return String.fromCharCode(x).toLowerCase();
});
var alphabetUppercase = alpha.map(function (x) { return String.fromCharCode(x); });
var alphabet = alphabetLowercase.concat(alphabetUppercase);
var alphabetMap = new Map();
for (var i = 0; i < alphabet.length; i++) {
    alphabetMap.set(alphabet[i], i + 1);
}
// part 1
var compartmentsForEachBackpack = parsedLines.map(function (line) {
    var lengthOfLine = line.length;
    var middleOfLine = Math.floor(lengthOfLine / 2);
    var firstHalf = line.slice(0, middleOfLine);
    var secondHalf = line.slice(middleOfLine, lengthOfLine);
    return [firstHalf, secondHalf];
});
var duplicatesInEachBackpack = compartmentsForEachBackpack.map(function (compartment) {
    var firstHalf = compartment[0], secondHalf = compartment[1];
    var firstHalfCharacters = firstHalf.split("");
    var secondHalfCharacters = secondHalf.split("");
    var duplicates = firstHalfCharacters.filter(function (character) {
        return secondHalfCharacters.includes(character);
    });
    return duplicates.slice(0, 1);
});
var duplicates = duplicatesInEachBackpack.flat();
var sumOfDuplicates = duplicates.reduce(function (acc, curr) {
    var value = alphabetMap.get(curr);
    return acc + value;
}, 0);
console.log("priorty of duplicates in compartments : ", sumOfDuplicates);
// part 2
function chunks(arr, n) {
    var i;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                i = 0;
                _a.label = 1;
            case 1:
                if (!(i < arr.length)) return [3 /*break*/, 4];
                return [4 /*yield*/, arr.slice(i, i + n)];
            case 2:
                _a.sent();
                _a.label = 3;
            case 3:
                i += n;
                return [3 /*break*/, 1];
            case 4: return [2 /*return*/];
        }
    });
}
var groupOfThreeBackpacks = Array.from(chunks(parsedLines, 3));
var commonItemsInEachGroup = groupOfThreeBackpacks.map(function (group) {
    var firstBackpack = group[0], secondBackpack = group[1], thirdBackpack = group[2];
    var firstBackpackCharacters = firstBackpack.split("");
    var secondBackpackCharacters = secondBackpack.split("");
    var thirdBackpackCharacters = thirdBackpack.split("");
    var commonItems = firstBackpackCharacters.filter(function (character) {
        return secondBackpackCharacters.includes(character);
    });
    var commonItemsInGroup = commonItems.filter(function (character) {
        return thirdBackpackCharacters.includes(character);
    });
    return commonItemsInGroup.slice(0, 1);
});
var commonItems = commonItemsInEachGroup.flat();
var sumOfCommonItems = commonItems.reduce(function (acc, curr) {
    var value = alphabetMap.get(curr);
    return acc + value;
}, 0);
console.log(sumOfCommonItems);
var hrend = process.hrtime(hrstart);
console.info("Execution time (hr): %ds %dms", hrend[0], hrend[1] / 1000000);
