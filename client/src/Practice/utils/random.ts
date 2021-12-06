export const rand_norm = (mean: number, variance: number) =>
  Math.sqrt(-2 * Math.log(1 - Math.random())) *
    Math.cos(2 * Math.PI * Math.random()) *
    variance +
  mean;

export const rand_range = (n: number) => Math.floor(Math.random() * n);
