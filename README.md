# Shallot

### How to run:
Run the following commands in the terminal with the Dockerfile:

```
docker build -t shallot .
docker run -dit --name shallot_container shallot
docker exec -it shallot_container bash
cd Shallot
cargo run
```

Open another terminal and connect to the container again using `docker exec -it shallot_container bash`, and within it, type the following command:

```
curl --proxy "http://127.0.0.1:7878" "https://www.facebook.com"
```

Initially, this will not work. It will return a 403 request because Facebook is not on the whitelist. Now type:

```
sed -i -e '$a*.*.*.*' whitelist.txt
```

This will add the wild card, and will make it so the whitelist accepts all connections. Type `curl --proxy "http://127.0.0.1:7878" "https://www.facebook.com"` again and the server will return the information from facebook.com where the curl request was passed. The terminal with the proxy server will note its connection. Event_log.txt will record everything printed out in the terminal, and log.txt will record the connection. Statistics.txt will check event_log.txt every 5 seconds to give summary information on logs.

### Crates used
* **Chrono:** Obtains datetime data.
* **URL:** An implementation for the URL standard.
* **Regex:** A library for regular expressions.
* **Public Suffix:** A library forMozilla's suffix.
* **HTTPParse:** A library for parsing HTTP requests.
* **Memcached:** A library for working with memcached, a memory-based approach to caching.

The following crates have been removed causing software conflicts.

* ~~**Hyper:** An HTTP library.~~ It did not allow enough control of the process.
* ~~**Tokio:** An asynchronous runtime.~~ It was causing our software to hang, so we found another solution.

### Deliverable 1

* We have implemented the single-threaded version of our proxy server. For the moment, it simply receives connections and logs them. In our next deliverable, we will modify the server to be multi-threaded so that it may handle simultaneous connections.
* Logging has also been implemented in its basic form. It currently logs the ip address and port of the connection, as well as the date and time that the connection was attempted. For deliverable 2, the functionality will be expanded to mark if the connection is incoming or outgoing, and it will also mention if the connection was flagged by the firewall. 
* We have developed a sorting module that currently organizes the log file in ascending order of IP address. For the next deliverable, we will modify that sorting to remove redundant IP addresses, as well as also sort by the incoming/outgoing and safe/unsafe dichotomies that the logging module will implement.

### Deliverable 2

* The server is now multi-threaded and capable of handling simultaneous connections. It does not currently have authentication; however, we plan to implement this feature for our final release.
* We have implemented a basic firewall. The firewall has a blacklist for outgoing connections, and a whitelist for incoming ones. The blacklist and whitelist files can be updated while the server is running, and it will account for these changes upon further requests. The server currently notes the connection attempts and links them to the whitelist or blacklist, but does not reject them; this will be added in the final deliverable. Additionally, we have basic payload probing to check if the request is in HTTP format, and simple checks to verify that the payload does not appear to be malicious.
* The logging module has been updated to include a recording of whether or not a request was accepted or rejected. If rejected, it will list the reason as it either being on the blacklist, not being on the whitelist, or flagged as untrusted, depending on the circumstances.

### Deliverable 3

* The server now has implementation in the form of the aforementioend firewall. Instead of simply checking the request, it is now properly rejected. We decided to not implement multi-layering, as the investment of development was not worth separating the whitelist and blacklist checks. Instead, we check them both on a single server.
* The blacklist and whitelist now reject lines not in the IPV4 format. They also allow for the use of wildcards. For instance, 172.0.0.* will match with 172.0.0.1, 172.0.0.2, and so on.
* We have also implemented caching in the form of memcached. During a server's runtime, whenever a curl request is sent, the outcome of that event will be saved. The cache is checked before actually sending the request, and returns the result if there is one.
* We have a statistics module that probes event_log.txt. It returns the number of connections, as well as the number of each type of event the server has encountered (e.g. denied because of the blacklist).
* We have basic payload probing to check if the request received is a valid HTTP request. We also have huge payload check. If the data transferred between the destination and source exceeds a certain limit, the request fails. A malicious user can also try to add multiple hosts to bypass host check . This is also tested by rejecting requests for certain blacklisted domains like .in, .pk, etc . Finally, we are also checking if the domain is an icann domain.
