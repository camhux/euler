var container = [0, 1]
var sum = 0

while container[1] < 4000000 {
  let next = container[0] + container[1]
  container[0] = container[1]
  container[1] = next
  if container[1] % 2 == 0 {
    sum += container[1]
  }
}

print("Sum below 4 million is \(sum)")