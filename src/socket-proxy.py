import socket
def connectThroughProxy():
    headers = """malicious code to connect to a server
                Host: www.google.com\r\n\r\n"""

    host = "127.0.0.1" #proxy server IP
    port = 7878              #proxy server port

    try:
        s = socket.socket()
        s.connect((host,port))
        s.send(headers.encode('utf-8'))
        response = s.recv(3000)
        print (response)
        s.close()
    except socket.error as m:
       print (str(m))
       s.close()
       sys.exit(1) 

connectThroughProxy()
