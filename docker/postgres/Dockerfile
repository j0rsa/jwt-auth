FROM postgres:12.1

ADD docker-entrypoint-initdb.d /docker-entrypoint-initdb.d
RUN chown postgres:postgres /docker-entrypoint-initdb.d/*
