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

MAN_PAGE := doc/yaqlite.1.txt
MAN_DIR := share/man/man1
MAN_PRODUCT := $(MAN_DIR)/yaqlite.1


.PHONY: doc
doc: $(MAN_PRODUCT)

$(MAN_PRODUCT): $(MAN_PAGE) | $(MAN_DIR)
	a2x --destination-dir $(MAN_DIR) --no-xmllint --format manpage $<

$(MAN_DIR):
	mkdir -p $@
