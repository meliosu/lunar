CC=cc

default: libadd.so

libadd.so: add.o
	$(CC) -shared -o $@ $<

add.o:
	$(CC) -c add.c
