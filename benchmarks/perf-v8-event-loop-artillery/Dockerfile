FROM artilleryio/artillery:2.0.9
RUN npm install -g \
    artillery-plugin-expect \
    artillery-plugin-apdex \
    artillery-plugin-metrics-by-endpoint \
    artillery-plugin-fake-data

RUN mkdir /reports
WORKDIR /scripts
COPY ./artillery.yaml .

CMD ["run", "-o", "/reports/report.json", "/scripts/artillery.yaml"]
