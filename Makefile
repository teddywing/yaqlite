# Copyright (c) 2022  Teddy Wing
#
# This file is part of Yaqlite.
#
# Yaqlite is free software: you can redistribute it and/or modify it
# under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# Yaqlite is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Yaqlite. If not, see <https://www.gnu.org/licenses/>.

VERSION := $(shell egrep '^version = ' Cargo.toml | head -n 1 | awk -F '"' '{ print $$2 }')
TOOLCHAIN := $(shell fgrep default_host_triple $(HOME)/.rustup/settings.toml | awk -F '"' '{ print $$2 }')

SOURCES := $(shell find src -name '*.rs')
RELEASE_PRODUCT := target/release/yaqlite

MAN_PAGE := doc/yaqlite.1.txt
MAN_DIR := share/man/man1
MAN_PRODUCT := $(MAN_DIR)/yaqlite.1

DIST := $(abspath dist)
DIST_PRODUCT := $(DIST)/bin/yaqlite
DIST_MAN_PAGE := $(DIST)/share/man/man1/yaqlite.1


$(RELEASE_PRODUCT): $(SOURCES)
	cargo build --release


.PHONY: doc
doc: $(MAN_PRODUCT)

$(MAN_PRODUCT): $(MAN_PAGE) | $(MAN_DIR)
	a2x --destination-dir $(MAN_DIR) --no-xmllint --format manpage $<

$(MAN_DIR):
	mkdir -p $@


.PHONY: dist
dist: $(DIST_PRODUCT) $(DIST_MAN_PAGE)

$(DIST):
	mkdir -p $@

$(DIST)/bin: | $(DIST)
	mkdir -p $@

$(DIST_PRODUCT): $(RELEASE_PRODUCT) | $(DIST)/bin
	cp $< $@

$(DIST_MAN_PAGE): share
	cp -R $< $(DIST)


.PHONY: pkg
pkg: yaqlite_$(VERSION)_$(TOOLCHAIN).tar.bz2

yaqlite_$(VERSION)_$(TOOLCHAIN).tar.bz2: dist
	tar cjv -s /dist/yaqlite_$(VERSION)_$(TOOLCHAIN)/ -f $@ dist
