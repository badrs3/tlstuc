#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// A simple function to test function calls
int add(int a, int b) {
    return a + b;
}

// A function to test string manipulation
char* greet(const char* name) {
    char* greeting = (char*)malloc(strlen(name) + 14); // "Hello, " + name + "!\n" + null terminator
    if (greeting == NULL) {
        return NULL;
    }
    
    strcpy(greeting, "Hello, ");
    strcat(greeting, name);
    strcat(greeting, "!\n");
    
    return greeting;
}

// A simple struct to test struct handling
typedef struct {
    int x;
    int y;
} Point;

// A function to test struct manipulation
Point create_point(int x, int y) {
    Point p;
    p.x = x;
    p.y = y;
    return p;
}

// Main function to run tests
int main() {
    // Test basic arithmetic
    printf("Testing basic arithmetic...\n");
    int sum = add(5, 7);
    printf("5 + 7 = %d\n", sum);
    
    // Test string manipulation
    printf("\nTesting string manipulation...\n");
    char* message = greet("tlstuc");
    printf("%s", message);
    free(message);
    
    // Test struct manipulation
    printf("\nTesting struct manipulation...\n");
    Point p = create_point(10, 20);
    printf("Point: (%d, %d)\n", p.x, p.y);
    
    // Test control flow
    printf("\nTesting control flow...\n");
    printf("Counting from 1 to 5: ");
    for (int i = 1; i <= 5; i++) {
        printf("%d ", i);
    }
    printf("\n");
    
    // Test array manipulation
    printf("\nTesting array manipulation...\n");
    int numbers[5] = {1, 2, 3, 4, 5};
    int sum_of_array = 0;
    for (int i = 0; i < 5; i++) {
        sum_of_array += numbers[i];
    }
    printf("Sum of array elements: %d\n", sum_of_array);
    
    printf("\nAll tests completed successfully!\n");
    
    return 0;
}