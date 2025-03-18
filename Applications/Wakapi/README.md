# Wakapi
[Wakapi](https://wakapi.dev/) is an open-source tool designed to track the time you spend coding on various projects in multiple programming languages and beyond.

Wakapi is an open-source alternative to [wakatime](https://wakatime.com/) that can run on-premises as a self-hosted solution.

## Usage

1. Run Wakapi on one of your devices, e.g. using [Docker](https://github.com/CSpyridakis/dockerfiles/blob/main/wakapi/docker-compose.yml).
2. Οpen VSCode and download the Wakatime extension.
We are going to use the Wakatime extension. Ηowever, we'll log into our server instead of the remote server.
3. Update the `Wakatime` configuration file `~/.wakatime.cfg` like this:
```
[settings]
api_url = https://<domain/ip>/api
api_key = <key>

status_bar_enabled = true
disabled = false
status_bar_coding_activity = true
debug = false
```