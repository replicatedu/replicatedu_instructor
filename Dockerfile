FROM ubuntu:latest


#installing rust
ENV RUSTUP_TOOLCHAIN=stable-x86_64-unknown-linux-musl
RUN cd && \
apt --assume-yes update && \
apt --assume-yes upgrade 
RUN apt --assume-yes install  curl ca-certificates gcc make openssl perl git gcc

#installing go
RUN apt --assume-yes install git make musl-dev golang
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

run curl -o /rustup-init.sh https://raw.githubusercontent.com/rust-lang-nursery/rustup.rs/master/rustup-init.sh && \
    sh /rustup-init.sh -y


RUN mkdir -p  /replicatedu
COPY * /replicatedu/



CMD ["make"]