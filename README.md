# rust_at_one
A small web project with the hopes of getting out on the web one day

## About
My intent is not to build something big, but instead just have a project to play around with.

### setup

.env file like 

`RUST_BACKTRACE=1;
IP_ADDRESS=0.0.0.0:8000
MONGODB_URI=mongodb://root:example@localhost:27017/
KEY_PEM=nopass.pem
CERT_PEM=cert.pem`

I'm using docker with a mongo instance for local development. run `docker-compose -f stack.yml up`.

to create a self-signed temporary cert for testing:
`openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
`openssl rsa -in key.pem -out nopass.pem`