test:
	cargo test -- --nocapture

test/ok: test/hello test/find_language

test/hello:
	cargo test --color=always --package cld3 --lib tests::implicit_constructor -- --exact

test/find_language:
	cargo test --color=always --package cld3 --lib tests::find_language -- --exact

test/find_topn_most_freq_langs:
	cargo test --color=always --package cld3 --lib tests::find_topn_most_freq_langs -- --exact