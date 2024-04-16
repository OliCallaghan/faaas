FROM artilleryio/artillery:2.0.9
RUN npm install -g \
    artillery-plugin-expect \
    artillery-plugin-apdex \
    artillery-plugin-metrics-by-endpoint \
    artillery-plugin-fake-data

WORKDIR /scripts
COPY ./artillery.yaml .

CMD ["run", "/scripts/artillery.yaml"]
