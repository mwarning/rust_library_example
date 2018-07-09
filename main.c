#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

struct Bar;

extern struct Bar *dat_new();
extern int dat_open(struct Bar* ptr);
extern void dat_free(struct Bar* ptr);
extern int dat_write(struct Bar *ptr, size_t offset, size_t length, const uint8_t* array);
extern int dat_read(struct Bar *ptr, size_t offset, size_t length, uint8_t* array);


int main(int argc, char **argv)
{
  struct Bar *ptr = dat_new();

  if (dat_open(ptr) == 0) {
    uint8_t in_data[6] = {1,2,3,4,5,6};
    uint8_t out_data[6] = {0};

    printf("Write vector to Rust lib:\n");
    dat_write(ptr, 0, sizeof(in_data), in_data);

    printf("Read vector back from Rust lib:\n");
    dat_read(ptr, 0, sizeof(out_data), out_data);

    // Print vector
    printf("Got: ");
    for (int i = 0; i < 6; i++) {
      printf("%d ", (int) out_data[i]);
    }
    printf("\n");
  }

  dat_free(ptr);

  return 0;
}
