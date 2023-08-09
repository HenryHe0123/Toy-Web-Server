## HTTP Format

Request format:

```
Method Request-URI HTTP-Version CRLF 
headers CRLF 
message-body
```

example:

```
Get / HTTP/1.1
```

Response format:

```
HTTP-Version Status-Code Reason-Phrase CRLF 
headers CRLF 
message-body
```

example:

```
HTTP/1.1 200 OK\r\n\r\n
```

