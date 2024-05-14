# sgpasswd - suckless password generator utility

.POSIX:

include config.mk

all: sgpasswd

sgpasswd: $(SRC)
	$(CC) build --release
	
install: sgpasswd
	cp $(TARGETDIR)/sgpasswd $(DESTDIR)

clean:
	rm -rf $(TARGETDIR)/sgpasswd

uninstall:
	rm $(DESTDIR)/sgpasswd
