export async function preparePayload(_requestParams, vuContext, _events) {
  const payload = {
    region: randomRegion(),
    year: randomNumberOfDays(),
  }
  vuContext.vars['payload'] = payload;
}

function randomNumberOfDays() {
  return Math.floor(Math.random() * (1996 - 1993)) + 1993
}

function randomRegion() {
  const regions = ["AFRICA", "AMERICA", "ASIA", "EUROPE", "MIDDLE EAST"];

  return regions[Math.floor(Math.random() * regions.length)];
}
