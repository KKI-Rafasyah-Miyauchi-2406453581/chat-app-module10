# Module 10 - Async Chat Application

## Experiment 2.1: Original code, and how it run

![Original Chat App](static/exp2_1_original.png)

**How to run and what happens:**
To run the application, I opened multiple terminal instances. In the first terminal, I started the server using `cargo run --bin server`. In the other terminals, I started the clients using `cargo run --bin client`. When text is typed into one client and submitted, it is sent to the server over the websocket. The server receives this message and broadcasts it out asynchronously to all other currently connected clients, which immediately prints the message to their respective screens.