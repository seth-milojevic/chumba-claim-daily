# claim-daily: Chumba Casino daily reward claimer

A simple script to automatically claim rewards from the popular gambling website Chumba Casino.

- [Usage Instructions](#usage-instructions)
- [Docker Instructions](#docker-instructions)
- [Cron Instructions](#cron-instructions)

## Usage Instructions:

1. Clone the repository

2. Within the repository, create the following file: `users.json`

3. Within `users.json`, provide the login(s) in JSON format like so:

```
{
    "users": [
        {"email": "EMAIL@DOMAIN.COM","password": "PASSWORD"},
        {"email": "EMAIL@DOMAIN.COM","password": "PASSWORD"},
        {"email": "EMAIL@DOMAIN.COM","password": "PASSWORD"}
    ]
}
```

Done! Feel free to `cargo run` and see the script go.

Below are optional steps for the following:
- [Running it within a docker container](#docker-instructions)
- [Scheduling a cron job to have it run every day](#cron-instructions)

## Docker Instructions:

1. Navigate into the repository

`cd /path/to/cloned/repository`

2. Build the docker image and name it "chumba-claim-daily".

`docker build -t chumba-claim-daily .`

3. Run the docker image we just created

`docker run --rm chumba-claim-daily:latest`

Voila! Docker will create and run a container with our claim-daily running inside.

## Cron Instructions

There will be two sets of cron instructions:
- [Cron job running the script directly on the host machine](#host-machine)
- [Cron job runnint the script within a docker container](#docker)

[!NOTE]
Both methods will utilize the crontab of the host machine. The only difference is where the claim-daily script is executed.

### Host Machine

1. Open crontab

`crontab -e`

2. Add the following to your crontab to execute the script every day at 5am

`0 5 * * * cd /path/to/cloned/repository && cargo run --release`

Done! Every day at 5am, `cargo run --release` will be executed within the repository's directory and run the script.

### Docker

I have personally had issues with running docker commands from within crontab, so I create shell scripts containing the docker commands I need crontab to run.

1. Create a shell script such as `docker-run.sh` and add

`docker run --rm chumba-claim-daily:latest`

2. Open crontab

`crontab -e`

3. Add the following to your crontab to execute the script every day at 5am

`0 5 * * * /bin/sh /path/to/shell/script/docker-run.sh`

4. Save your changes and exit crontab

Done! Every day at 5am, the container will be created and run the script.
