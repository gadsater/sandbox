#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <math.h>
#include <libguile.h>

static const int WIDTH = 10;
static const int HEIGHT = 10;

static FILE* global_output;

static double x, y;
static double direction;
static int pendown;

static FILE *start_gnuplot() {
	FILE* output;
	int pipes[2];
	pid_t pid;

	pipe(pipes);
	pid = fork();

	if (!pid) {
		dup2(pipes[0], STDIN_FILENO);
		execlp("gnuplot", NULL);
		return NULL;
	}

	output = fdopen(pipes[1], "w");

	fprintf(output, "set multiplot\n");
	fprintf(output, "set parametric\n");
	fprintf(output, "set xrange [-%d:%d]\n", WIDTH, WIDTH);
	fprintf(output, "set yrange [-%d:%d]\n", HEIGHT, HEIGHT);
	fprintf(output, "set size ratio -1\n");
	fprintf(output, "unset xtics\n");
	fprintf(output, "unset ytics\n");
	fflush(output);

	return output;
}

static void draw_line(FILE* output, double x1, double y1, double x2, double y2) {
	fprintf(output, "plot[0:1] %f + %f * t, %f + %f *t notitle\n",
					x1, x2 - x1, y1, y2 - y1);
	fflush(output);
}

static SCM turtle_reset() {
	x = y = 0.0;
	direction = 0.0;
	pendown = 1;

	fprintf(global_output, "clear\n");
	fflush(global_output);

	return SCM_UNSPECIFIED;
}

static SCM turtle_pendown() {
	SCM result = scm_from_bool(pendown);
	pendown = 1;
	return result;
}

static SCM turtle_penup() {
	SCM result = scm_from_bool(pendown);
	pendown = 0;
	return result;
}

static SCM turtle_turn(SCM degrees) {
	const double value = scm_to_double(degrees);
	direction += M_PI / 180 * value;
	return scm_from_double(direction * 180.0 / M_PI);
}

static SCM turtle_move(SCM length) {
	const double value = scm_to_double(length);
	double newX, newY;

	newX = x + value * cos(direction);
	newY = y + value * sin(direction);

	if (pendown)
		draw_line(global_output, x, y, newX, newY);

	x = newX;
	y = newY;
	return scm_list_2(scm_from_double(x), scm_from_double(y));
}

static SCM turtle_quit() {
	fprintf(global_output, "quit\n");
	return SCM_UNSPECIFIED;
}

static void *register_functions(void *data) {
	scm_c_define_gsubr("turtle-reset", 0, 0, 0, &turtle_reset);
	scm_c_define_gsubr("turtle-penup", 0, 0, 0, &turtle_penup);
	scm_c_define_gsubr("turtle-pendown", 0, 0, 0, &turtle_pendown);
	scm_c_define_gsubr("turtle-turn", 1, 0, 0, &turtle_turn);
	scm_c_define_gsubr("turtle-move", 1, 0, 0, &turtle_move);
	return NULL;
}

int main(int argc, char* argv[]) {
	global_output = start_gnuplot();
	turtle_reset();

	turtle_pendown();
	scm_with_guile(&register_functions, NULL);
	scm_shell(argc, argv);
	
	return EXIT_SUCCESS;
}


