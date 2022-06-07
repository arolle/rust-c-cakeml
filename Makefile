# Cargo package name
RUSTNAME:=rustccakeml

RUSTLIBDIR:=target/debug
# for Linux
RUSTFILE:=lib$(RUSTNAME).so
RUSTLIBPATH:=$(RUSTLIBDIR)/$(RUSTFILE)

.PHONY: all
all: run

.PHONY: run
run: even_numbers
	env RUST_BACKTRACE=full LD_LIBRARY_PATH=$(RUSTLIBDIR) ./$<

$(RUSTLIBPATH): src/lib.rs
	cargo build

%: %.S basis_ffi.c $(RUSTLIBPATH)
	$(CC) $< basis_ffi.c -o $@ -L$(RUSTLIBDIR) -l$(RUSTNAME)

%.S: %.cml
	$(CAKEML) <$< >$@

%.S: %.sexp
	$(CAKEML) \
		--skip_type_inference=true \
		--exclude_prelude=true \
		--sexp=true \
		< $< > $@

%.sexp: %Script.sml
	Holmake

