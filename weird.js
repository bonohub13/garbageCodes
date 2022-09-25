#!/bin/node

function number(n) {
    if (n === +[]) {
        return +[];
    } else {
        let num = +[];

        while (n >= (+[] + !+[])) {
            num = num + +[] + !+[];
            n = n - (+[] + !+[]);
        }

        return num;
    }
}

function neg(num) {
    let neg_one = +[] - (+[] + !+[]);
    return number(num) * neg_one;
}

console.log(number(10))
console.log(number(10.11111111111111111111))
console.log(number(10) ** number(10))
console.log(neg(100) * number(0))
