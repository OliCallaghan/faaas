export async function entrypoint(event, ctx) {
  if ("rmqMessagesByQueue" in event) {
    const {rmqMessagesByQueue} = event;

    if ("task_invocations::/" in rmqMessagesByQueue) {
      const msgs = rmqMessagesByQueue["task_invocations::/"];

      for (const msg of msgs) {
        console.log(atob(msg.data))
      }
    }
  }
  return "Hello world"
}
