#!/bin/bash
echo "Make sure OpenSSL and Docker is installed on this machine"
if [ ! -f ".env" ]; then
    echo "Creating your .env files for local dev using cargo commands"

    # File used when running cargo run
    echo "RUST_BACKTRACE=1;
MONGODB_URI=mongodb://kronk:5RwjSCNN@localhost:27017/
DB_NAME=rust_at_one

USE_SSL=true
KEY_PEM=certs/nopass.pem
CERT_PEM=certs/cert.pem

USE_LE=false
LE_EMAIL=NotUsed
LE_DOMAIN=NotUsed" > .env

    # File used for integration testing
    echo "RUST_BACKTRACE=1;
MONGODB_URI=mongodb://kronk:5RwjSCNN@localhost:27017/
DB_NAME=rust_at_one_test

USE_SSL=true
KEY_PEM=certs/nopass.pem
CERT_PEM=certs/cert.pem

USE_LE=false
LE_EMAIL=NotUsed
LE_DOMAIN=NotUsed" > test.env
fi

if [ ! -d "certs" ]; then
    echo "Setting up self signed certificates"
    mkdir certs
    openssl req -x509 -newkey rsa:4096 -nodes -keyout ./certs/key.pem -out ./certs/cert.pem -days 365 -subj '/CN=localhost'
    openssl rsa -in ./certs/key.pem -out ./certs/nopass.pem
fi

echo "Running the application locally using stack.yml"
docker-compose -f stack.yml up -d

echo "Testing the setup"
id=$(curl -s -k -d '{"name":"¡no see!", "author":"The script"}' \
        -X POST 'https://localhost:8000/api/posts' -H "Content-Type: application/json" | jq -r .id )

curl -k -i 'https://localhost:8000/api/posts/$id'

curl -k -i -d '{ "name":"updated!" }' \
    -X PUT 'https://localhost:8000/api/posts/$id' -H "Content-Type: application/json"

curl -k -i \
    -X DELETE 'https://localhost:8000/api/posts/$id' -H "Content-Type: application/json"

echo -e "GET(404): 'https://localhost:8000/api/posts/$id'"
curl -k -i 'https://localhost:8000/api/posts/$id'

curl -s -k -d '{"name":"One", "author":"The script"}' -X POST 'https://localhost:8000/api/posts' -H "Content-Type: application/json"
curl -s -k -d '{"name":"Two", "author":"The script"}' -X POST 'https://localhost:8000/api/posts' -H "Content-Type: application/json"
curl -s -k -d '{"name":"Three", "author":"The script"}' -X POST 'https://localhost:8000/api/posts' -H "Content-Type: application/json"
curl -s -k -d '{"name":"Four", "author":"The script"}' -X POST 'https://localhost:8000/api/posts' -H "Content-Type: application/json"
curl -k -i 'https://localhost:8000/api/posts?name=One&number=0&count=2'

echo "check localost:8081, there should be posts in rust_at_one"