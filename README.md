yaqlite
=======

A bridge between YAML and SQLite. Enables inserting YAML records into a
database and selecting records as YAML.


## Usage
	$ sqlite3 test.db <<EOF
		CREATE TABLE "restaurants" (
			id INTEGER PRIMARY KEY,

			name TEXT,
			description TEXT,
			rating INTEGER
		);
	EOF

	$ yaqlite insert --database test.db restaurants <<EOF
	name: Western Restaurant Nekoya
	description: >-
	  Located in the business district, this restaurant serves a wide variety of
	  cuisine for all tastes and appetites.

	  Enjoy hearty dishes like the beef stew, and conclude with a chocolate parfait
	  so light and airy you'll think you're eating a cloud.
	rating: 5
	EOF

	$ yaqlite select --database test.db restaurants 1
	---
	name: Western Restaurant Nekoya
	description: |
	  Located in the business district, this restaurant serves a wide variety of cuisine for all tastes and appetites.
	  Enjoy hearty dishes like the beef stew, and conclude with a chocolate parfait so light and airy you'll think you're eating a cloud.
	rating: 5


## Install
Mac OS X users can install with MacPorts, after [adding a custom repository
source][teddywing ports repository]:

	$ sudo port install yaqlite


[teddywing ports repository]: https://github.com/teddywing/macports-ports#adding-this-repository-source


## License
Copyright © 2022 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).
