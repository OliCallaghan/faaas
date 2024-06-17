export async function preparePayload(_requestParams, vuContext, _events) {
  const ms = randomMsDelay();
  const payload = { ms };

  // console.log(ms)

  vuContext.vars['payload'] = payload;
}

function randomMsDelay() {
  const currentMinutes = new Date().getMinutes();

  if (currentMinutes % 30 > 15)
    return 200;

  return 50;
}
