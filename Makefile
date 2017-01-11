install_dev:
	sudo pacman -S xorg-server-xnest

dev:
	cargo build --features "xlib xinput"
	Xephyr -screen 800x600 :1 &
	DISPLAY=:1 sleep .5 &
	DISPLAY=:1 urxvt &
	DISPLAY=:1 sxhkd &
	DISPLAY=:1 ./target/debug/wm
