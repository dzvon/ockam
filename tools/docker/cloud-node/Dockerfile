# Stage 1 - Build elixir release of ockam_cloud_node elixir app
FROM ghcr.io/build-trust/ockam-builder@sha256:60e2c1e45d54e31b50f3e56fe202276ecc4812ccac88c84c9af8910cf787a81d as elixir-app-release-build
COPY . /work
RUN set -xe; \
    cd implementations/elixir; \
    ../../gradlew build; \
    cd ockam/ockam_cloud_node; \
    MIX_ENV=prod mix release;


# TODO: Use distroless container after https://github.com/elixir-lang/elixir/issues/11942 is closed
# Stage 2 - Create container and copy executables in above step
FROM debian:11.1-slim@sha256:312218c8dae688bae4e9d12926704fa9af6f7307a6edb4f66e479702a9af5a0c

COPY --from=elixir-app-release-build /work/implementations/elixir/ockam/ockam_cloud_node/_build/prod/rel/ockam_cloud_node /opt/ockam_cloud_node

ENV LANG=C.UTF-8

EXPOSE 4000

ENTRYPOINT ["/opt/ockam_cloud_node/bin/ockam_cloud_node"]
CMD ["start"]
