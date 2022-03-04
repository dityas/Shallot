# Shallot

### How to run:
Run the following commands in the terminal with the Dockerfile:

```
docker build -t shallot .
docker run -p 7878:7878 -dit --name shallot_container shallot
docker exec -it shallot_container bash
cargo run
```

Open another terminal and connect to the container again using `docker exec -it [container name] bash`, and within it, type the following command a few times:

```
curl 127.0.0.1:7878
```

curl will return empty replies for now, but looking at the terminal running the server will show that the connections have been received.
Note that log.txt has logged the connections (along with some pre-written sample data), and sortedLogs.txt contains them sorted by ip address in ascending order.

### Crates used
* **Chrono:** Used for obtaining datetime data.

### Deliverable 1

* We have implemented the single-threaded version of our proxy server. For the moment, it simply receives connections and logs them. In our next deliverable, we will modify the server to be multi-threaded so that it may handle simultaneous collections.
* Logging has also been implemented in its basic form. It currently logs the ip address and port of the connection, as well as the date and time that the connection was attempted. For deliverable 2, the functionality will be expanded to mark if the connection is incoming or outgoing, and it will also mention if the connection was flagged by the firewall.
* We have developed a sorting module that currently organizes the log file in ascending order of IP address. For the next deliverable, we will modify that sorting to remove redundant IP addresses, as well as also sort by the incoming/outgoing and safe/unsafe dichotomies that the logging module will implement.
