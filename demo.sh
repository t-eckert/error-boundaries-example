#!/bin/sh

url="http://localhost:3000"

curl -s -X POST $url/users -H "Content-Type: application/json" -d '{"name": "Thomas", "pass": "1234"}'

curl -s -X POST $url/accounts -H "Content-Type: application/json" -H "Password: 1234" -d '{"name": "Thomas"}'
