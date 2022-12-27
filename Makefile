ifeq ($(CAKE),)
CAKE:=$(CAKEDIR)/cake
endif

BASIS:=$(CAKEDIR)/basis_ffi.c

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

%: %.S $(BASIS) ffi_extra.c $(RUSTLIBPATH)
	$(CC) $< $(BASIS) ffi_extra.c \
		-o $@ -L$(RUSTLIBDIR) -l$(RUSTNAME)

%.S: %.cml
	$(CAKE) <$< >$@

%.S: %.sexp
	$(CAKE) \
		--skip_type_inference=true \
		--exclude_prelude=true \
		--sexp=true \
		< $< > $@

%.sexp: %Script.sml
	Holmake

