#include <stdio.h>
#include <stdlib.h>

struct Person {
    char name[50];
    int age;
};

int main() {
    struct Person *people[2];

    struct Person p1 = {"Alice", 54};
    people[0] = &p1;

    struct Person *p2 = malloc(sizeof(struct Person));
    snprintf(p2->name, sizeof(p2->name), "Bob");
    p2->age = 43;
    people[1] = p2;

    for (int i = 0; i < 2; i++) {
        printf("%s (%d)\n", people[i]->name, people[i]->age);
    }

    for (struct Person **p = people; p < &people[2]; p++) {
        printf("%s (%d)\n", (*p)->name, (*p)->age);
    }

    free(p2);

    return 0;
}
