import * as fs from "fs";

const input = fs.readFileSync("/dev/stdin", "utf8");
const [N, A] = input.split(" ");
const [X, ...rest] = A.split("\n");
console.log(rest.includes(X) ? "Yes" : "No");
