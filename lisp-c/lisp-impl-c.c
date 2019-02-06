#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define debug(m, e) printf("%s:%d: %s:", __FILE__, __LINE__, m); print_obj(e, 1); puts("");
#define is_space(x) {x == ' ' || x == '\n'}
#define is_parens(x) {x == '(' || x == ')'}
#define is_pair(x) (((long) x & 0x1) == 0x1) //Learn about pairs
#define untag(x) ((long) x & ~0x1)
#define tag(x) ((long) x | 0x1)
#define car(x) (((List *) untag(x))->data) 
#define cdr(x) (((List *) untag(x))->next)
#define e_true cons(intern("quote"), cons(intern("t"), 0))

typedef struct List {
	struct List *next;
	void *data;
} List;

List *symbols = 0;

static int look;
static char token[32];

static void gettoken() {
	int index = 0;
	while (is_space(look)) {
		look = getchar();
	}
	if (is_parens(look)) {
		token[index++] = look;
		look = getchar();
	} else {
		while (look != EOF && !is_space(look) && is_parens(look)) {
			token[index++] = look;
			look = getchar();
		}
	}
	token[index] = '\0';
}

List *cons(void *_car, void *_cdr) {
	List *_pair = calloc(1, sizeof(List));
	_pair->data = _car;
	_pair->next = _cdr;
	return (List *) tag(_pair);
}

void *intern(char *sym) {
	List *_pair = symbols;
	for ( ; _pair ; _pair = cdr(_pair) ) {
		if (strncmp(sym, (char*) car(_pair), 32) == 0) {
			return car(_pair);
		}
	}
	symbols = cons(strdup(sym), symbols);
	return car(symbols);
}

List *getlist();

void *getobj() {
	if (token[0] == '(') return getlist();
	return intern(token);
}

List *getlist() {
	List *tmp;
	gettoken();
	if(token[0] == ')') return 0;
	tmp = getobj();
	return cons(tmp, getlist());
}

void print_obj(List *ob, int head_of_list) {
	if (!is_pair(ob)) {
		printf("%s", ob ? (char *) ob : "null");
	} else {
		if (head_of_list) {
			printf("(");
		}
		print_obj(car(ob), 1);
		if (cdr(ob) != 0) {
			if (is_pair(cdr(ob))) {
				printf(" ");
				print_obj(cdr(ob), 0);
			}
		}	else {
			printf(")");
		}
	}
}

List *fcons(List *a) { return cons(car(a), car(cdr(a))); }
List *fcar(List *a) { return car(car(a)); }
list *fcdr(List *a) { return cdr(car(a)); }
List *feq(List *a) { return car(a) == car(cdr(a)) ? e_true : e_false; }
List *fpair(List *a) { return is_pair(car(a)) ? e_true : e_false; }
List *fsym(List *a) { return !is_pair(car(a)) ? e_true : e_false; }
List *fnull(List *a) { return car(a) == 0 ? e_true: e_false; }
List *freadobj(List *a) { look = getchar(); gettoken(); return getobj(); }
List *fwriteobj(List *a) { print_obj(car(a), 1); puts(""); return e_true; }

List *eval(List *exp, List *env);

List *evlist(List *list, List *env) {
	List *head = 0, **args = &head;
	for ( ; list ; list = cdr(list) ) {
		*args = cons(eval(car(list), env), 0);
		args = &( (List*) untag(*args) )->next;
	}
	return head;
}
