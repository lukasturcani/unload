FROM otel/opentelemetry-collector-contrib:latest
COPY otel.yaml /etc/otel-collector-config.yaml
ENTRYPOINT ["/otelcol-contrib"]
CMD ["--config", "/etc/otel-collector-config.yaml"]
