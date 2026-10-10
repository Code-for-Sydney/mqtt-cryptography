using System;
using System.Text;
using System.Threading;
using MQTTnet;
using MQTTnet.Client;

var factory = new MqttFactory();
var client = factory.CreateMqttClient();

var options = new MqttClientOptionsBuilder()
    .WithTcpServer("broker.hivemq.com", 1883)
    .Build();

client.ApplicationMessageReceivedAsync += e =>
{
    var msg = Encoding.UTF8.GetString(e.ApplicationMessage.Payload).Trim();
    if (!string.IsNullOrEmpty(msg))
    {
        Console.WriteLine(msg);
    }
    return Task.CompletedTask;
};

await client.ConnectAsync(options);
await client.SubscribeAsync("1");

while (true)
{
    await client.PublishAsync("1", "echo");
    Thread.Sleep(1000);
}
