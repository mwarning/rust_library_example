#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

struct RandomAccessMethods;

extern struct RandomAccessMethods *dat_new();
extern void dat_free(struct RandomAccessMethods* ptr);

extern void dat_write(
  struct RandomAccessMethods *self, size_t offset, size_t length, const uint8_t* array
);

extern void dat_read(
  struct RandomAccessMethods *self, size_t offset, size_t length, uint8_t* array
);


int main(int argc, char **argv)
{
  struct RandomAccessMethods *ptr = dat_new();

  uint8_t in_data[6] = {1,2,3,4,5,6};
  dat_write(ptr, 0, sizeof(in_data), in_data);

  uint8_t out_data[6] = {0};
  dat_read(ptr, 0, sizeof(out_data), out_data);

  // print out data
  for (int i = 0; i < 6; i++) {
    printf("%d ", (int) out_data[i]);
  }
  printf("\n");

  dat_free(ptr);

  return 0;
}
