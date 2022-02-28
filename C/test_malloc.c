#include<stdlib.h>
#include<stdio.h>
#include <unistd.h>
#include <string.h>
#include <malloc.h>

#define EXP1_NUM_MALLOCS 2000
#define EXP1_MALLOC_SIZE 100
#define EXP2_NUM_MALLOCS 100
#define EXP2_MALLOC_SIZE 2048
#define EXP3_NUM_MALLOCS 100
#define EXP3_MALLOC_SIZE 2048
#define EXP5_LARGE_OBJECT_SIZE 1024
#define EXP6_MALLOC_SIZE 1000

void experiment6() {
    char *p = malloc(EXP6_MALLOC_SIZE);

    // Write to addresses outside of the allocated region until it segfaults
    // Can you find the offset (value of i) where it segfaults??
    // Hint: use gdb to run the program and print local variables after the segfault
    //
    // TODO 4: use gdb to find the value of 'i' when the below for loop hits a SEGFAULT
    // Hint: use 'info locals' in gdb
    for(int i=0;i<1000000;i++) {
        *(p + EXP6_MALLOC_SIZE + i) = 'a';
    }
}

void experiment5() {
    // malloc a large enough object so that its address starts with 0x7...
    /* TODO 3: increase EXP5_LARGE_OBJECT_SIZE until you find a size large enough */
    //void *p = malloc(EXP5_LARGE_OBJECT_SIZE);
    // 40960 resulted in heap allocation
    // 134480 resulted in mmap allocation
    void *p = malloc(134480);
    free(p);
    printf("Large object is at %p\n", p);
}

void experiment4() {
    void *p, *p2;

    // Malloc two objects of the same size
    // The addresses of the objects will be the same
    p = malloc(100);
    free(p);
    p2 = malloc(100);
    free(p2);
    printf("Object 1 at '%p'. Object 2 is at '%p'\n", p, p2);

    // Malloc two objects of different sizes
    // The addresses of both objects will be different
    p = malloc(100);
    free(p);
    p2 = malloc(200);
    free(p2);
    printf("Object 1 at '%p'. Object 2 is at '%p'\n", p, p2);
}

void experiment3() {
    // This is the address of the end of heap at the start of this experiment
    void *start_heap = sbrk(0);
    void *prev_heap = sbrk(0);
    void *curr_heap;
    void *pointers[EXP3_NUM_MALLOCS];

    // Allocate many objects of the same size
    for (int i=0;i<EXP3_NUM_MALLOCS;i++) {
        pointers[i] = malloc(EXP3_MALLOC_SIZE);

        // Check to see if the heap has changed
        curr_heap = sbrk(0);
        if (curr_heap != prev_heap) {
            printf("Heap changed from %p to %p after %d allocations\n", prev_heap, curr_heap, i+1);
            printf("Heap size change %ld bytes\n", curr_heap - prev_heap);
        }
        prev_heap = curr_heap;
    }

    printf("Total heap change after allocations: %ld bytes\n", curr_heap - start_heap);

    // Deallocate all chunks in reverse order of allocation.
    //
    // Question:
    //   At which deallocation in the below loop does the heap size finally reduce?
    //   Compare this to the result of experiment2. Why do you think there is a difference?
    //   Hint: since we're deallocating in reverse order, we free objects closer to the end of the
    //   heap first.
    start_heap = sbrk(0);
    for (int i=EXP3_NUM_MALLOCS-1;i>=0;i--) {
        free(pointers[i]);

        // Check to see if the heap has changed
        curr_heap = sbrk(0);
        if (curr_heap != prev_heap) {
            printf("Heap changed from %p to %p after deallocating the object at index %d\n", prev_heap, curr_heap, i);
            printf("Heap size change %ld bytes\n", prev_heap - curr_heap);
        }
        prev_heap = curr_heap;
    }
    printf("Total heap change after deallocations: %ld bytes\n", start_heap - curr_heap);
}


void experiment2() {
    void *start_heap = sbrk(0);
    void *prev_heap = sbrk(0);
    void *curr_heap;
    void *pointers[EXP2_NUM_MALLOCS];

    // Allocate many objects of the same size
    for (int i=0;i<EXP2_NUM_MALLOCS;i++) {
        pointers[i] = malloc(EXP2_MALLOC_SIZE);

        // Check to see if the heap has changed
        curr_heap = sbrk(0);
        if (curr_heap != prev_heap) {
            printf("Heap changed from %p to %p after %d allocations\n", prev_heap, curr_heap, i+1);
            printf("Heap size increased by %ld bytes\n", curr_heap - prev_heap);
        }
        prev_heap = curr_heap;
    }

    printf("Total heap change after allocations: %ld bytes\n", curr_heap - start_heap);

    // Deallocate all chunks in order of allocation.
    //
    // Question:
    //   At which deallocation in the below loop does the heap size finally reduce?
    start_heap = sbrk(0);
    for (int i=0;i<EXP2_NUM_MALLOCS;i++) {

        // free releases an a region of memory that was allocated earlier
        // Read: https://man7.org/linux/man-pages/man1/free.1.html
        // Question:
        //   To free a region of memory, what argument should be passed to free?
        //   Fill in TODO 2 with the correct argument once you've figured it out.
        //   Hint: we're trying to free the object pointed to by pointers[i]
        free(pointers[i]);

        // Check to see if the heap has changed
        curr_heap = sbrk(0);
        if (curr_heap != prev_heap) {
            printf("Heap changed from %p to %p deallocating object at index %d\n", prev_heap, curr_heap, i);
            printf("Heap size decreased by %ld bytes\n", prev_heap - curr_heap);
        }
        prev_heap = curr_heap;
    }
    printf("Total heap change after all deallocations: %ld bytes\n", start_heap - curr_heap);
}

void experiment1() {
    // start_heap is the address of the end of the heap at the start of this experiment
    // Read https://linux.die.net/man/2/sbrk to understand what sbrk does
    // Question:
    //   Why does passing in 0 to sbrk return the end of the heap?
    void *start_heap = sbrk(0); 

    // The loop below continuosly allocates memory of a fixed size
    // 
    // The local variables, prev_heap and curr_heap track the address of the end of the heap
    // through each iteration. curr_heap is used to store the end of the heap after the current allocation
    // and prev_heap stores the heap end of the last allocation. We then compare the old and new heap size
    // to figure out when the heap changes.
    //
    // Try to answer the following:
    //   At which allocation does the heap size first change?
    //   Is the change in heap size greater than or equal to the size of the allocated object?
    void *prev_heap = sbrk(0);
    void *curr_heap;
    for (int i=0;i<EXP1_NUM_MALLOCS;i++) {

        // malloc allocates a region of memory of the provided size
        // Read: https://man7.org/linux/man-pages/man3/malloc.3.html
        //
        // Question:
        //   What does malloc return?
        malloc(EXP1_MALLOC_SIZE);
        /* TODO 1: replace NULL with the correct function to get current heap end */
        curr_heap = sbrk(0);

        // Check to see if the heap has changed
        if (curr_heap != prev_heap) {
            printf("Heap changed from %p to %p after %d allocations\n", prev_heap, curr_heap, i+1);
            printf("Heap size increased by %ld bytes\n", curr_heap - prev_heap);
            prev_heap = curr_heap;
        }
    }

    printf("Total heap change: %ld bytes\n", curr_heap - start_heap);
}

void print_usage() {
    printf("invalid input\n\nUsage: test_malloc <experiment number>\n\nExample: ./test_malloc 5\nThe above runs example 5.\n");
    exit(0);
}

int main(int argc, char *argv[]) {
    
    if (argc < 2 || strlen(argv[1]) != 1) {
        print_usage();
    }

    switch(argv[1][0]) {
    case '1':
        experiment1();
        break;
    case '2':
        experiment2();
        break;
    case '3':
        experiment3();
        break;
    case '4':
        experiment4();
        break;
    case '5':
        experiment5();
        break;
    case '6':
        experiment6();
        break;
    default:
        print_usage();
    }

    return 0;
}