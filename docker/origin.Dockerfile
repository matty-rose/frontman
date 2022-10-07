FROM golang:1.18 AS build
WORKDIR /go/src/github.com/matty-rose/frontman/demos/origin
COPY go.mod .
COPY go.sum .
RUN go mod download && go mod verify
COPY main.go main.go
RUN CGO_ENABLED=0 GOOS=linux go build -a -o bin/app main.go


FROM scratch as prod
WORKDIR /root/
COPY --from=build /go/src/github.com/matty-rose/frontman/demos/origin/bin/app .
EXPOSE 8000
CMD ["./app"]
