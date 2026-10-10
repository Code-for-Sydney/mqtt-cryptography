
import paho.mqtt.client as mqtt

def on_message(client, userdata, message):
    msg = message.payload.decode('utf-8').strip()
    if msg:
        print(msg)

client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
client.connect("broker.hivemq.com", 1883)
client.subscribe("1")
client.on_message = on_message

client.loop_start()

while True:
    client.publish("1", "echo")
