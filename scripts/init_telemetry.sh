#!/usr/bin/env bash

docker run \
 -d \
 --name rcps-assets2-jaeger \
 -p5778:5778/tcp -p5775:5775/udp -p6831:6831/udp -p6832:6832/udp -p16686:16686 \
 jaegertracing/jaeger-agent:1.30 --reporter.grpc.host-port=collector.aspecto.io:14250 --reporter.grpc.tls.enabled=true --agent.tags=aspecto.token=6ff4c22a-fd6b-4603-b6da-be23aa52890f


