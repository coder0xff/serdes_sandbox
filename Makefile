SOURCE=src/source.rs
MAIN=src/main.rs
TMP=src/tmp.rs

.PHONY: all
all:
	cp $(SOURCE) $(MAIN)
	cargo build
	touch $(MAIN)
	RUSTC_BOOTSTRAP=1 cargo rustc -- -Zunpretty=expanded >$(TMP)
	mv $(TMP) $(MAIN)
	cargo fmt
	cargo build