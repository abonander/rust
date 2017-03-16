# The ICE occurred in the following situation:
# * `foo` declares `extern crate bar, baz`, depends only on `bar` (forgetting `baz` in `Cargo.toml`)
# * `bar` declares and depends on `extern crate baz`
# * All crates built in metadata-only mode (`cargo check`)
all:
	$(RUSTC) baz.rs --emit=metadata --out-dir=$(TMPDIR)
	$(RUSTC) bar.rs --emit=metadata --extern baz=$(TMPDIR)/libbaz.rmeta --out-dir=$(TMPDIR)
	$(RUSTC) foo.rs --emit=metadata -L dependency=. --extern bar=$(TMPDIR)/libbar.rmeta --out-dir=$(TMPDIR) 2>&1 | \
	grep -vq "unexpectedly panicked"
	# ^ Succeeds if it doesn't find the ICE message
