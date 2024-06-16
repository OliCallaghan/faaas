export async function preparePayload(_requestParams, vuContext, _events) {
  const payload = {
    partSize: randomPartSize(),
    partType: randomPartType(),
    regionName: randomRegionName()
  }
  vuContext.vars['payload'] = payload;
}

function randomPartSize() {
  return Math.floor(Math.random() * 50) + 1
}

function randomPartType() {
  const allowedPartType = ["TIN", "NICKEL", "BRASS", "STEEL", "COPPER"]
  return allowedPartType[Math.floor(Math.random() * allowedPartType.length)]
}

function randomRegionName() {
  const allowedRegions = ["AFRICA", "AMERICA", "ASIA", "EUROPE", "MIDDLE EAST"]
  return allowedRegions[Math.floor(Math.random() * allowedRegions.length)]
}
