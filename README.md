# Shallot

### How to run:
Run the following commands in the terminal with the Dockerfile:

```
docker build -t shallot .
docker run -dit --name shallot_container shallot
docker exec -it shallot_container bash
cargo run
```

Open another terminal and connect to the container again using `docker exec -it shallot_container bash`, and within it, type the following command:

```
curl 1.2.3.4
```

Note that this request will not be accepted, as it is part of the blacklist. Now type (this is one of Google's IP addresses):

```
curl 64.233.177.100
```

This request will be accepted. However, if you place the same IP in blacklist.txt and press enter, then attempt to curl the same IP address, it will be rejected, demonstrating that the blacklist can be updated while the server is running.

### Crates used
* **Chrono:** Obtains datetime data.
* **Hyper:** An HTTP library.
* **Tokio:** An asynchronous runtime.
* **URL:** An implementation for the URL standard.
* **Regex:** A library for regular expressions.

### Deliverable 1

* We have implemented the single-threaded version of our proxy server. For the moment, it simply receives connections and logs them. In our next deliverable, we will modify the server to be multi-threaded so that it may handle simultaneous connections.
* Logging has also been implemented in its basic form. It currently logs the ip address and port of the connection, as well as the date and time that the connection was attempted. For deliverable 2, the functionality will be expanded to mark if the connection is incoming or outgoing, and it will also mention if the connection was flagged by the firewall.
* We have developed a sorting module that currently organizes the log file in ascending order of IP address. For the next deliverable, we will modify that sorting to remove redundant IP addresses, as well as also sort by the incoming/outgoing and safe/unsafe dichotomies that the logging module will implement.

### Deliverable 2

* The server is now multi-threaded and capable of handling simultaneous connections. It does not currently have authentication; however, we plan to implement this feature for our final release.
* We have implemented a basic firewall. The firewall has a blacklist for outgoing connections, and a whitelist for incoming ones. The blacklist and whitelist files can be updated while the server is running, and it will account for these changes upon further requests. Additionally, we have basic payload probing to check if the request is in HTTP format, and simple checks to verify that the payload does not appear to be malicious.
* The logging module has been updated to include a recording of whether or not a request was accepted or rejected. If rejected, it will list the reason as it either being on the blacklist, not being on the whitelist, or flagged as untrusted, depending on the circumstances.