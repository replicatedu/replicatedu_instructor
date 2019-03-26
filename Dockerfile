FROM alpine:latest


#installing rust
ENV RUSTUP_TOOLCHAIN=stable-x86_64-unknown-linux-musl
RUN cd && \
apk update && \
apk upgrade && \
apk add curl ca-certificates gcc make zlib-dev openssl-dev perl git cargo rust make

#installing go
RUN apk add --no-cache git make musl-dev go
# Configure Go
ENV GOROOT /usr/lib/go
ENV GOPATH /go
ENV PATH /go/bin:$PATH

RUN mkdir -p ${GOPATH}/src ${GOPATH}/bin

#WORKDIR $GOPATH

#install hub
RUN mkdir -p "$GOPATH"/src/github.com/github
RUN git clone \
  --config transfer.fsckobjects=false \
  --config receive.fsckobjects=false \
  --config fetch.fsckobjects=false \
  https://github.com/github/hub.git "$GOPATH"/src/github.com/github/hub
RUN cd "$GOPATH"/src/github.com/github/hub
RUN go install github.com/github/hub

#install latest version of cargo and rustup
RUN cd ~/
RUN cd && git clone --depth 1 https://github.com/rust-lang-nursery/rustup.rs.git && cd rustup.rs && cargo build --release
RUN echo "export PATH=$PATH:~/.cargo/bin"  >> ~/.profile && source ~/.profile && \
apk del cargo && \
apk del rust && \
~/rustup.rs/rustup-init.sh -y


RUN mkdir -p  /replicatedu
COPY * /replicatedu



CMD ["make"]