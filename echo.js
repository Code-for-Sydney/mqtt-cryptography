const mqtt = require("mqtt");

const client = mqtt.connect("mqtt://broker.hivemq.com");

client.on("connect", () => {
  client.subscribe("1");
  setInterval(() => {
    client.publish("1", "echo");
  }, 1000);
});

client.on("message", (_topic, message) => {
  const msg = message.toString().trim();
  if (msg) {
    console.log(msg);
  }
});
