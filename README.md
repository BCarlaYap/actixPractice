#### To run the server
```sh
cargo run --bin server
```
it will display the initial secret key.

### To run the client
Given the initial secret key, run this locally:
```sh
cargo run --bin client_secret <secret_key>
```
It will allow 127.0.0.1 to connect again without the secret key.
Run this command:
```sh
cargo run --bin client
```
#### Access WebSocket From Other Device
* through secret key: `ws://<server_ip_address>:8080/secret/<secret_key>`

* if the device's ip address is already in the whitelist: `ws://<server_ip_address>:8080/success/`

### Add IP 
`POST http://localhost:8080/whitelist`
Set in the body a json:
```json
{
    "ip_address": "<ip_address>"
}
```
This will respond with a newly generated secret key.

### Remove IP
`DELETE http://localhost:8080/whitelist`
Set in the body a json:
```json
{
    "ip_address": "<ip_address>"
}
```
Returns `true` if removed successfully.

### Display the whitelist
Access from another port: `GET http://0.0.0.0:8090/whitelist`
