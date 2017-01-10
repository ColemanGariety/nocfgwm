install_dev:
	sudo pacman -S xorg-server-xnest

dev:
	cargo build --features "xlib xinput"
	Xephyr -screen 800x600 :1 &
	DISPLAY=:1 ./target/debug/wm &
	DISPLAY=:1 xterm &
	DISPLAY=:1 sxhkd
