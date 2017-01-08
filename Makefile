install_dev:
	sudo pacman -S xorg-server-xnest

dev:
	cargo build --features "xlib xinput"
	Xnest :1 -geometry 1024x768+0+0 &
	DISPLAY=:1 ./target/debug/wm
