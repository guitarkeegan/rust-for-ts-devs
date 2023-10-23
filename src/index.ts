function myFunk(n: number | undefined): number | undefined {
  if (!n) return (n ?? undefined)
  return n*5;
}

console.log(myFunk(undefined));
console.log(myFunk(5));