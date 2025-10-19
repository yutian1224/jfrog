FROM --platform=$TARGETPLATFORM alpine:3.22

RUN apk add --no-cache curl
COPY ./jfrog_supervisor /jfrog_supervisor

ENV MALLOC_CONF="tcache:true,tcache_max:65536,narenas:16,lg_chunk:24,dirty_decay_ms:30000,muzzy_decay_ms:30000,max_background_threads:16"
ENTRYPOINT ["/jfrog_supervisor"]
