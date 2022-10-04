In order to run the e2e tests you need to start a local aca-py instance. This
can be done by executing the following in the repo root:

```sh
docker-compose -f ./docker/docker-compose.acapy.min.yml up
```

As a future enhancement to e2e we can have the test harness set up the entire
environment for us.

