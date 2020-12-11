FROM golang AS build

COPY . .

RUN go get -u github.com/notnil/chess

RUN go build main.go

FROM alpine AS runtime

COPY --from=build /go/main .

CMD ["./main"]