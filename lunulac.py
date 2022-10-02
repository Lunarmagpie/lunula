import socket
import sys

with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
    s.connect("/tmp/lunula-socket.socket")
    s.sendall(' '.join(sys.argv[1:]).encode("utf-8"))
