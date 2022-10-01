import socket

with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
    s.connect("/tmp/lunula-socket.socket")
    s.sendall(b"Hello, world")
