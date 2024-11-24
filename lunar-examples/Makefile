CC=gcc
FLAGS=-Wall -Wextra -g
TARGETS=libadd.so

default: $(TARGETS)

add.o: add.c
	$(CC) $(FLAGS) -c $<

libadd.so: add.o
	$(CC) $(FLAGS) -shared -o $@ $<

clean:
	rm -f $(TARGETS) *.o
