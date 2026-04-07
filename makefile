all:
	@cargo build --release

clean:
	@cargo clean

install:
	@mkdir -p /etc/hrustfetch/assets/
	@cp assets/* /etc/hrustfetch/assets/
	@cp target/release/hrustfetch /usr/bin/hrustfetch

uninstall:
	@rm /usr/bin/hrustfetch
	@rm -rf /etc/hrustfetch/assets
