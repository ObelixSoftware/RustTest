# RustTest

Throw a HTTP 400 if input is invalid: Invalid transform type. Must be 'uppercase' or 'lowercase'

```bash
curl --location 'http://127.0.0.1:8080/transform' \
--header 'Content-Type: application/json' \
--data '{
"transform": "33",
"html": "<p>Hello world</p>"
}'
```

## Uppercase multiple neasted paragraphs

```bash
curl --location 'http://127.0.0.1:8080/transform' \
--header 'Content-Type: application/json' \
--data '{
    "transform" : "uppercase",
    "html": "<div><p>First paragraph</p><span>Not a paragraph</span><p>Second paragraph</p></div>"
}'
```

## Nested

```bash
curl --location 'http://127.0.0.1:8080/transform' \
--header 'Content-Type: application/json' \
--data '
{
"transform": "uppercase",
"html": "<p>Text with <strong>bold</strong> and <em>italic</em> elements</p>"
}'
```