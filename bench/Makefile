PERL?=perl

.PHONY: xs rs bench

bench: xs rs
	$(PERL) compare.pl

rs: rs/Makefile
	make -C $@
xs: xs/Makefile
	make -C $@

%/Makefile: %/Makefile.PL
	(cd $(dir $@) && $(PERL) Makefile.PL)

clean:
	make -C rs distclean
	make -C xs distclean
