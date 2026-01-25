CC ?= cc
TARGET ?= syscall2333

.PHONY: all install clean

all: main.c
	$(CC) -static -o $(TARGET) main.c

install: all
	mkdir -p $(DADK_CURRENT_BUILD_DIR)
	cp $(TARGET) $(DADK_CURRENT_BUILD_DIR)/$(TARGET)

clean:
	rm -f $(TARGET) *.o
