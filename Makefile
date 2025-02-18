.PHONY: all clean format check test docs update publish

all:
	cargo build --lib --release

clean:
	cargo clean
	@rm -rf pkg target*

format:
	cargo fmt

check:
	cargo clippy -- -W clippy::all -W clippy::correctness -W clippy::suspicious -W clippy::complexity -W clippy::perf -W clippy::style -W clippy::pedantic -W clippy::panic -A clippy::doc_markdown -A clippy::wildcard_imports -A clippy::module_name_repetitions -D warnings

test:
	cargo test -- --nocapture

docs:
	cargo rustdoc --release --all-features -- -A rustdoc::broken_intra_doc_links
	echo "<meta http-equiv=\"refresh\" content=\"0;url=opus_sys/index.html\" />" > target/doc/index.html

update:
	cargo update
	git submodule update
	git add opus
	git commit -m "opus submodule updated"
	git push

publish:
	cargo publish
