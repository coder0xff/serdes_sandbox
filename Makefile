SOURCE=src/source.rs
MAIN=src/main.rs
TMP=src/tmp.rs

.PHONY: all
all:
	cp $(SOURCE) $(MAIN)
	RUSTC_BOOTSTRAP=1 cargo build
	touch $(MAIN)
	RUSTC_BOOTSTRAP=1 cargo rustc -- -Zunpretty=expanded >$(TMP)
	mv $(TMP) $(MAIN)
	RUSTC_BOOTSTRAP=1 cargo fmt
	RUSTC_BOOTSTRAP=1 cargo build

.PHONY: clean
clean:
	cargo clean
	rm src/main.rs
