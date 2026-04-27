TODO: impl client

Using ncat to test server... (for more information: https://nmap.org/ncat/guide/index.html)

```bash
$ nc 127.0.0.1 9999
server: got connection from client 127.0.0.1:42340.
Server is ready...
REVERSE
200 OK
divad
david
.
s: david
s: divad
```

Same connection from the server side:

```bash
     Running `target/debug/prj2_server`
server: got connection from client 127.0.0.1:42340.
127.0.0.1:42340 sends REVERSE
```


> In this project you need to write a client and a server program to implement a
> very simple application level protocol. Your client and server should work as
> follows. Your server runs first and then waits for connections from your
> client  on a port number you choose. You client talks to the server  by sending
> commands UPPERCASE, LOWERCASE, REVERSE, and EXIT to the server. Initially, when
> the server accepts the connection from a client, the server should first reply
> with the message "server: got connection from client x.x.x.x" where x.x.x.x is
> the IP address of the client. Then, It should reply with the message "Server is
> ready...".  Both messages are displayed at the client side. The followings
> illustrate the protocol in detail.


UPPERCASE

This command requires the server to transform the ASCII text sent from the client to all upper-letter text. The ASCII text sent from the client is ended with "."  in the last line.  The server should reply "200 OK" and then return the text in uppercase to the client. A client-server interaction looks like (shown at the client side):

c: UPPERCASE
s: 200 OK
c: Hello.
c: Nice to meet you!
c: .
s: HELLO.
s: NICE TO MEET YOU!


At the server side, the server should display "x.x.x.x sends UPPERCASE" when receiving this command. For all other commands, the server should also do this kind of display.

LOWERCASE

This command is similar to the previous command except that the server sends lower case text back to client.

c: LOWERCASE
s: 200 OK
c: HELLO.
c: NICE TO MEET YOU!
c: .
s: hello.
s: nice to meet you!


REVERSE

When the server receives this command, it sends  to the client the received message in the reverse order.

c: REVERSE
s: 200 OK

c: HELLO
s: OLLEH


EXIT

This command closes the connection. After receiving this command, the server should respond the acknowledgement "200 OK" to the client, display the "x.x.x.x sends EXIT", then close itself.

A client-server interaction thus looks like:

c: EXIT
s: 200 OK


If  the server receives an invalid command, it should sends an error message to the client "400: Not a valid command!".
