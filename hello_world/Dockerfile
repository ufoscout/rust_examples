FROM scratch

# By default rust binaries needs gclib at runtime. This is not needed when using the musl target.
# this is an example of an alpine linux image with glib:
#FROM alpine:3.4
#RUN apk --allow-untrusted --no-cache -X http://apkproxy.heroku.com/andyshinn/alpine-pkg-glibc add glibc glibc-bin

ADD ./target/x86_64-unknown-linux-musl/release/hello_world /

CMD ["/hello_world"]
