const fs = require('fs');
const path = require('path');

const inputFilePath = path.join(__dirname, 'day3_input.txt');
const input_string = fs.readFileSync(inputFilePath, 'utf-8');

const regex = /mul\(\d{1,3},\d{1,3}\)/g;
const matches = input_string.match(regex);

console.log('Number of matches: ', matches.length);

const table = matches.map(match => {
    const matchResult = match.match(/mul\((\d{1,3}),(\d{1,3})\)/);
    if (matchResult) {
        const [_, num1, num2] = matchResult;
        return { num1: parseInt(num1), num2: parseInt(num2) };
    }
    return null;
}).filter(item => item !== null);

if (matches.length !== table.length) {
    throw new Error('Mismatch between number of matches and table entries');
}

const tableWithProduct = table.map(entry => ({
    ...entry,
    product: entry.num1 * entry.num2
}));

// console.table(tableWithProduct);

// Print the sum of the product column
const sum = tableWithProduct.reduce((acc, curr) => acc + curr.product, 0);
console.log('Part one answer: ', sum);

// Part two
const regex_2 = /mul\(\d{1,3},\d{1,3}\)/g;
const matches_2 = [];
let inDoBlock = false;

// const test_string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

while ((match = regex_2.exec(input_string)) !== null) {
    const substring = input_string.substring(0, match.index);
    const doIndex = substring.lastIndexOf("do()");
    const dontIndex = substring.lastIndexOf("don't()");

    if (dontIndex === -1) {
        inDoBlock = true;
    } else if (doIndex > dontIndex) {
        inDoBlock = true;
    } else if (dontIndex > doIndex) {
        inDoBlock = false;
    }

    if (inDoBlock) {
        matches_2.push(match[0]);
    }
}

console.log('Number of matches while enabled: ', matches_2.length);

// console.log(matches_2);

const table_2 = matches_2.map(match => {
    const matchResult = match.match(/mul\((\d{1,3}),(\d{1,3})\)/);
    if (matchResult) {
        const [_, num1, num2] = matchResult;
        return { num1: parseInt(num1), num2: parseInt(num2) };
    }
    return null;
}).filter(item => item !== null);

if (matches_2.length !== table_2.length) {
    throw new Error('Mismatch between number of matches_2 and table_2 entries');
}

const tableWithProduct_2 = table_2.map(entry => ({
    ...entry,
    product: entry.num1 * entry.num2
}));

// console.table(tableWithProduct_2);

// Print the sum of the product column
const sum_2 = tableWithProduct_2.reduce((acc, curr) => acc + curr.product, 0);
console.log('Part two answer: ', sum_2);