all:
	@cargo build --release

clean:
	@cargo clean

install:
	@mkdir -p /etc/hrustfetch/assets/
	@mv assets/* /etc/hrustfetch/assets/
	@mv target/release/hrustfetch /usr/bin/hrustfetch

uninstall:
	@rm /usr/bin/hrustfetch
	@rm -rf /etc/hrustfetch/assets
