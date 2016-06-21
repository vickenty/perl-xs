#define PERL_NO_GET_CONTEXT
#include "EXTERN.h"
#include "perl.h"
#include "XSUB.h"

MODULE = XSBench::XS	PACKAGE = XSBench::XS

NV xs_sum(AV* array)
	INIT:
	NV sum = 0;
	SSize_t i, top = av_top_index(array);
	CODE:
	for (i = 0; i <= top; i++) {
		SV **sv = av_fetch(array, i, 0);
		if (sv != NULL) {
			sum += SvNV(*sv);
		}
	}
	RETVAL = sum;
	OUTPUT:
	RETVAL

IV xs_gcd(IV a, IV b)
	INIT:
	IV t;
	CODE:
	while (b > 0) {
		t = a;
		a = b;
		b = t % b;
	}
	RETVAL = a < 0 ? -a : a;
	OUTPUT:
	RETVAL
