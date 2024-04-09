FROM otel/opentelemetry-collector-contrib:latest
COPY otel.yaml /etc/otel-collector-config.yaml
ENTRYPOINT ["/otelcontribcol", "--config=/etc/otel-collector-config.yaml"]
