export async function preparePayload(_requestParams, vuContext, _events) {
  const payload = {
    days: randomNumberOfDays(),
  }
  vuContext.vars['payload'] = payload;
}

function randomNumberOfDays() {
  return Math.floor(Math.random() * (120 - 60)) + 60
}
