FROM rustlang/rust:nightly-bullseye AS build

COPY . /source
WORKDIR /source
RUN cargo build --release


FROM gcr.io/distroless/cc-debian11:nonroot AS final

WORKDIR /app
COPY --from=build /source/target/release/thermometer /app/thermometer
ENV PATH="/app:$PATH"
ENTRYPOINT ["/app/thermometer"]
