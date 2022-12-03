#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
    if (argc != 2) {
        printf("Usage: day2 <part>\n");
        exit(-1);
    }

    int part = strcmp(argv[1], "1") == 0 ? 1 : 2;

    char a, b;

    long score = 0;
    while (scanf("%c %c\n", &a, &b) != EOF) {
        int opp = a - 'A';
        int you;

        if (part == 1) {
            you = b - 'X';
        } else {
            switch (b) {
            case 'X': // Lose
                you = (opp + 2) % 3;
                break;
            case 'Y': // Draw
                you = opp;
                break;
            case 'Z': // Win
                you = (opp + 1) % 3;
                break;
            }
        }

        score += you + 1;
        if (you == opp) {
            score += 3;
        } else if ((you + 2) % 3 == opp) {
            score += 6;
        }
    }

    printf("Your score: %ld\n", score);
}