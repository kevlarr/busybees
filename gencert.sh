openssl req -x509 -newkey rsa:4096 -nodes -keyout localkey.pem -out localcert.pem -days 365 -subj '/CN=localhost'
