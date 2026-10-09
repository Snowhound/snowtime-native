# Load runs

`app.patch` is the 081 branch's change to the app's load harness, against the pinned
commit: `perf/stress` gains `--app=native`, `--direct`, replication, CPU sets, edge
configurations, and the sampler's extra counters, with their `deploy/compose` overlays and
`perf/README.md` text. `perf:stress --app=native` and `harness/scaling/` need it. The
patch builds the native image from this repository (`app/..`, or `NATIVE_PORT`) with
`deploy/Dockerfile`.

snowtime `main` hasn't taken the change, so the pinned `app/` lacks it. Apply it to the
submodule's working tree for a run, and restore it afterwards, since `app/` is never
edited in a commit here:

```sh
git -C app apply ../deploy/load/app.patch
(cd app && bun run perf:stress --app=native --recording=<file> --dataset=M --run=kinds)
git -C app checkout -- . && git -C app clean -fd -- perf deploy
```

Open (the porting kit's `docs/port-repository.md` expects the app's load harness to take
a server address or image): land a generic form of this change in snowtime, so the
harness takes the port's image and compose overlay instead of knowing the native backend.
Then this patch goes away.
