const timeout = (ms: number) => new Promise((res) => setTimeout(res, ms));

export async function echo(ms: number): Promise<string> {
  await timeout(ms);

  return "success local";
}

echo.proxy = "proxy.echo";
