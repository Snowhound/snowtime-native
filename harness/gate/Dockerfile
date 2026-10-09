FROM snowtime-kit-render:bench
COPY --from=grafana/k6:2.3.0 /usr/bin/k6 /usr/local/bin/k6
WORKDIR /repo
ENTRYPOINT ["bun", "harness/perf-gate.ts", "--inside"]
