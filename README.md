# Valet

As a valet in the real world parks cars in their appropriate places, so does this valet of mine scans the inbox directory and processes and parks various files into their appropriate destinations.

## External dependencies

It currently depends on the below programs being installed on the system and them being available in `PATH`

- __qpdf__ : Installed using `brew install qpdf`
- __pdftotext__ : Installed as part of xpdf. It is installed using `brew install xpdf`

## Current capabilities

Currently this can handle the below files

- __Credit card statements:__ It decrypts them using `qpdf` and then appropriately renames and files them in their respective directories. It also uses `pdftotext` to read contents in a case to identify the statement date.
