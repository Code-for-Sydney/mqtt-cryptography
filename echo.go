package main

import (
	"fmt"
	"strings"
	"time"

	MQTT "github.com/eclipse/paho.mqtt.golang"
)

func main() {
	opts := MQTT.NewClientOptions().AddBroker("tcp://localhost:1883")

	client := MQTT.NewClient(opts)
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}

	client.Subscribe("1", 0, func(client MQTT.Client, msg MQTT.Message) {
		m := strings.TrimSpace(string(msg.Payload()))
		if m != "" {
			fmt.Println(m)
		}
	})

	for {
		client.Publish("1", 0, false, "echo")
		time.Sleep(time.Second)
	}
}
