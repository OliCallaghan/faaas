type Weibull3Params = [number, number, number];

export function weibull3(
  threshold: number,
  scale: number,
  shape: number,
): Readonly<Weibull3Params> {
  return [threshold, scale, shape];
}

export function weibull3pdf(
  [threshold, scale, shape]: Readonly<Weibull3Params>,
  val: number,
) {
  if (val < threshold) return 0;

  return (
    (shape / scale) *
    Math.pow((val - threshold) / scale, shape - 1) *
    Math.exp(-Math.pow((val - threshold) / scale, shape))
  );
}

export function weibull3cdf(
  [threshold, scale, shape]: Readonly<Weibull3Params>,
  val: number,
) {
  if (val < threshold) return 0;

  return 1 - Math.exp(-Math.pow((val - threshold) / scale, shape));
}

export function weibull3median([
  threshold,
  scale,
  shape,
]: Readonly<Weibull3Params>) {
  return scale * Math.pow(Math.LN2, 1 / shape) + threshold;
}
