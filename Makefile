MAN_PAGE := doc/yaqlite.1.txt
MAN_DIR := share/man/man1
MAN_PRODUCT := $(MAN_DIR)/yaqlite.1


.PHONY: doc
doc: $(MAN_PRODUCT)

$(MAN_PRODUCT): $(MAN_PAGE) | $(MAN_DIR)
	a2x --destination-dir $(MAN_DIR) --no-xmllint --format manpage $<

$(MAN_DIR):
	mkdir -p $@
