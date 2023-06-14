FROM rustlang/rust:nightly-alpine3.16 as build
RUN apk add lld openssl-dev libc-dev --no-cache
RUN cargo install cargo-leptos

FROM alpine:3.16 as css
RUN apk add npm --no-cache
RUN npm i -g pnpm
COPY site /site
WORKDIR /site
RUN pnpx tailwindcss -i ./input.scss -o ./style/output.css

FROM build
COPY site /site
COPY --from=css site/style /site/style
RUN cargo leptos build --release

FROM alpine:3.16
ENV LEPTOS_OUTPUT_NAME="leptos_start"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="127.0.0.1:3000"
RUN mkdir /app
COPY --from=build /site/target/site /app/site
COPY --from=build /site/target/server/leptos_start /app/leptos_start
CMD /app/leptos_start
