# Nacho teh Cat Discord Bot

## Run it from a container
Install it using docker. Execute these commands insode the cloned repo folder.
```
sudo docker build -t nacho_bot:0.3.0 .
docker run -d -e PORT=8080 -e DISCORD_TOKEN="<your-token-discord-bot>" -p 8080:8080 nacho_bot:0.3.0
```

Port 8080 is only to show if the app is runnign and healthy

## Run it directly from the code

Build it and run it directly from the folder
```
cargo b
DISCORD_TOKEN="<your-token-discord-bot>" cargo r
```

## reference for discord message format

https://birdie0.github.io/discord-webhooks-guide/discord_webhook.html