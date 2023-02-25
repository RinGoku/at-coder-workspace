import * as fs from "fs";

const input = fs.readFileSync("/dev/stdin", "utf8");
const [N, A, B] = input.split(" ").map((i) => Number(i));
console.log(B <= N * A ? B : N * A);
