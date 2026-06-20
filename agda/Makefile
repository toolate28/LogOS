# TriWeavon Agda formal layer — build, check, and HTML docs
AGDA ?= agda
LIB  := TriWeavon.agda-lib
SRC  := src
DOCS := docs/generated
EVERYTHING := $(SRC)/Everything.agda

.PHONY: all vendor check html clean

all: vendor check

vendor:
	@test -d vendor/agda-cubical/.git || git clone --depth 1 https://github.com/agda/cubical.git vendor/agda-cubical
	@test -d vendor/agda-stdlib/.git || git clone --depth 1 https://github.com/agda/agda-stdlib.git vendor/agda-stdlib

check: vendor
	$(AGDA) -l $(LIB) $(EVERYTHING)

html: vendor
	mkdir -p $(DOCS)
	$(AGDA) -l $(LIB) --html --html-dir=$(DOCS) $(EVERYTHING)

clean:
	rm -rf $(DOCS)/*.html $(DOCS)/Agda.css