var container = [0, 1];
var sum = 0;
var temp;

while (container[1] < 4000000) {
  temp = container[0] + container[1];
  container[0] = container[1];
  container[1] = temp;

  if (container[1] % 2 === 0) {
    sum += container[1];
  }
}

console.log(`Sum below 4 million is ${sum}`);