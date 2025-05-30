/**
 * features.c - Demonstrates various C language features supported by tlstuc
 *
 * This example program showcases different C language features that are
 * supported by the tlstuc compiler and runtime.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Macro definitions
#define MAX_BUFFER_SIZE 1024
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define SQUARE(x) ((x) * (x))

// Global variables
int global_var = 42;
const char* program_name = "features";

// Structure definition
typedef struct {
    int id;
    char name[50];
    float score;
} Student;

// Enumeration
typedef enum {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
} Weekday;

// Function prototypes
void print_student(const Student* student);
Student create_student(int id, const char* name, float score);
int factorial(int n);
int fibonacci(int n);
char* weekday_to_string(Weekday day);

/**
 * Main function
 */
int main(int argc, char** argv) {
    printf("tlstuc Features Demo\n");
    printf("====================\n\n");
    
    // Command line arguments
    printf("Command Line Arguments:\n");
    printf("Program name: %s\n", program_name);
    printf("Number of arguments: %d\n", argc);
    for (int i = 0; i < argc; i++) {
        printf("Argument %d: %s\n", i, argv[i]);
    }
    printf("\n");
    
    // Variables and data types
    printf("Variables and Data Types:\n");
    char c = 'A';
    int i = 123;
    float f = 3.14f;
    double d = 2.71828;
    bool b = true;
    
    printf("char: %c\n", c);
    printf("int: %d\n", i);
    printf("float: %f\n", f);
    printf("double: %lf\n", d);
    printf("bool: %s\n", b ? "true" : "false");
    printf("global_var: %d\n", global_var);
    printf("\n");
    
    // Operators
    printf("Operators:\n");
    int a = 10, b = 3;
    printf("%d + %d = %d\n", a, b, a + b);
    printf("%d - %d = %d\n", a, b, a - b);
    printf("%d * %d = %d\n", a, b, a * b);
    printf("%d / %d = %d\n", a, b, a / b);
    printf("%d %% %d = %d\n", a, b, a % b);
    printf("%d & %d = %d\n", a, b, a & b);
    printf("%d | %d = %d\n", a, b, a | b);
    printf("%d ^ %d = %d\n", a, b, a ^ b);
    printf("~%d = %d\n", a, ~a);
    printf("%d << %d = %d\n", a, b, a << b);
    printf("%d >> %d = %d\n", a, b, a >> b);
    printf("\n");
    
    // Control flow
    printf("Control Flow:\n");
    
    // If-else
    int x = 42;
    if (x > 50) {
        printf("%d is greater than 50\n", x);
    } else if (x > 30) {
        printf("%d is between 31 and 50\n", x);
    } else {
        printf("%d is 30 or less\n", x);
    }
    
    // Switch
    Weekday today = WEDNESDAY;
    switch (today) {
        case MONDAY:
            printf("Today is Monday\n");
            break;
        case TUESDAY:
            printf("Today is Tuesday\n");
            break;
        case WEDNESDAY:
            printf("Today is Wednesday\n");
            break;
        case THURSDAY:
            printf("Today is Thursday\n");
            break;
        case FRIDAY:
            printf("Today is Friday\n");
            break;
        case SATURDAY:
        case SUNDAY:
            printf("Today is the weekend\n");
            break;
        default:
            printf("Invalid day\n");
            break;
    }
    
    // For loop
    printf("For loop: ");
    for (int i = 0; i < 5; i++) {
        printf("%d ", i);
    }
    printf("\n");
    
    // While loop
    printf("While loop: ");
    int j = 0;
    while (j < 5) {
        printf("%d ", j);
        j++;
    }
    printf("\n");
    
    // Do-while loop
    printf("Do-while loop: ");
    int k = 0;
    do {
        printf("%d ", k);
        k++;
    } while (k < 5);
    printf("\n\n");
    
    // Arrays
    printf("Arrays:\n");
    int numbers[5] = {1, 2, 3, 4, 5};
    printf("numbers: ");
    for (int i = 0; i < 5; i++) {
        printf("%d ", numbers[i]);
    }
    printf("\n");
    
    char name[] = "tlstuc";
    printf("name: %s\n", name);
    printf("name length: %zu\n", strlen(name));
    printf("\n");
    
    // Pointers
    printf("Pointers:\n");
    int value = 42;
    int* ptr = &value;
    printf("value: %d\n", value);
    printf("&value: %p\n", (void*)&value);
    printf("ptr: %p\n", (void*)ptr);
    printf("*ptr: %d\n", *ptr);
    
    *ptr = 100;
    printf("After *ptr = 100:\n");
    printf("value: %d\n", value);
    printf("*ptr: %d\n", *ptr);
    printf("\n");
    
    // Dynamic memory allocation
    printf("Dynamic Memory Allocation:\n");
    int* dynamic_array = (int*)malloc(5 * sizeof(int));
    if (dynamic_array != NULL) {
        for (int i = 0; i < 5; i++) {
            dynamic_array[i] = i * 10;
        }
        
        printf("dynamic_array: ");
        for (int i = 0; i < 5; i++) {
            printf("%d ", dynamic_array[i]);
        }
        printf("\n");
        
        free(dynamic_array);
    }
    printf("\n");
    
    // Structures
    printf("Structures:\n");
    Student student1 = create_student(1, "Alice", 95.5f);
    Student student2 = create_student(2, "Bob", 87.0f);
    
    print_student(&student1);
    print_student(&student2);
    printf("\n");
    
    // Enumerations
    printf("Enumerations:\n");
    for (Weekday day = MONDAY; day <= SUNDAY; day++) {
        printf("%s\n", weekday_to_string(day));
    }
    printf("\n");
    
    // Functions
    printf("Functions:\n");
    printf("factorial(5) = %d\n", factorial(5));
    printf("fibonacci(10) = %d\n", fibonacci(10));
    printf("MIN(10, 5) = %d\n", MIN(10, 5));
    printf("SQUARE(4) = %d\n", SQUARE(4));
    printf("\n");
    
    printf("Demo completed successfully!\n");
    return 0;
}

/**
 * Print student information
 */
void print_student(const Student* student) {
    printf("Student ID: %d, Name: %s, Score: %.1f\n", 
           student->id, student->name, student->score);
}

/**
 * Create a new student
 */
Student create_student(int id, const char* name, float score) {
    Student student;
    student.id = id;
    strncpy(student.name, name, sizeof(student.name) - 1);
    student.name[sizeof(student.name) - 1] = '\0'; // Ensure null termination
    student.score = score;
    return student;
}

/**
 * Calculate factorial recursively
 */
int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

/**
 * Calculate Fibonacci number
 */
int fibonacci(int n) {
    if (n <= 0) {
        return 0;
    }
    if (n == 1) {
        return 1;
    }
    
    int a = 0, b = 1, c;
    for (int i = 2; i <= n; i++) {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}

/**
 * Convert weekday enum to string
 */
char* weekday_to_string(Weekday day) {
    switch (day) {
        case MONDAY: return "Monday";
        case TUESDAY: return "Tuesday";
        case WEDNESDAY: return "Wednesday";
        case THURSDAY: return "Thursday";
        case FRIDAY: return "Friday";
        case SATURDAY: return "Saturday";
        case SUNDAY: return "Sunday";
        default: return "Unknown";
    }
}