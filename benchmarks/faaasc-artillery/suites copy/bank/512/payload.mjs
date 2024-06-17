export async function preparePayload(_requestParams, vuContext, _events) {
  const src = randomAccount();
  const dst = nextAccount(src);

  const payload = {
    src,
    dst,
    amount: randomAmount()
  }
  vuContext.vars['payload'] = payload;
}

function randomAccount() {
  return Math.floor(Math.random() * 13) + 1
}

function nextAccount(acc) {
  return (acc % 13) + 1
}

function randomAmount() {
  return Math.floor(Math.random() * 50)
}
