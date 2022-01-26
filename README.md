# rust-api-architecture-sample

copying of a sutra.

- https://blog.foresta.me/posts/rust-layered-architecture/
- https://blog.j5ik2o.me/entry/2021/12/08/000000

Thanks.

## useage

### create .env file

```sh
$ echo DATABASE_URL=mysql://sample:sample@127.0.0.1:13306/sample > .env
```

### wake up MySQL

```sh
$ docker-compose up
```

### wake up Http Server

```sh
$ cargo run
```

### access to hello world!

```sh
$ curl -i -X GET http://127.0.0.1:8080
HTTP/1.1 200 OK
content-length: 12
date: Wed, 26 Jan 2022 13:59:02 GMT

Hello World!%
```

### post documents

```sh
$ curl -i -X POST -H "Content-Type: application/json" -d '{"title":"sample title", "body":"sample body"}' http://127.0.0.1:8080/documents
HTTP/1.1 200 OK
content-length: 35
content-type: application/json
date: Wed, 26 Jan 2022 13:59:46 GMT

{"id":"01FTBAYV6PYPSZYG6EXC4ES0NZ"}%
```
